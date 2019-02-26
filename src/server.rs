use crate::hinge::Hinge;
use crate::hinge::HingeState;
use crate::lock::Lock;
use crate::lock::LockState;
use crate::util::Result;
use rocket::config::Config;
use rocket::config::Environment;
use rocket::http::Status;
use rocket::State;
use rocket_contrib::json::Json;
use std::sync::Arc;
use std::sync::Mutex;

#[derive(Deserialize)]
pub struct ServerSettings {
    pub mount_point: String,
    pub port: u16,
}

#[derive(Deserialize)]
struct LockInput {
    state: Option<LockState>,
}

#[derive(Serialize)]
struct LockOutput {
    state: LockState,
    last_change: u64,
}

#[derive(Serialize)]
struct HingeOutput {
    state: HingeState,
}

pub fn run(settings: ServerSettings, hinge: Arc<Mutex<Hinge>>, lock: Arc<Mutex<Lock>>) {
    rocket::custom(
        Config::build(Environment::Staging)
            .address("localhost")
            .port(settings.port)
            .unwrap(),
    )
    .mount(
        &settings.mount_point,
        routes![put_lock, put_lock_toggle, get_lock, get_hinge],
    )
    .manage(hinge)
    .manage(lock)
    .launch();
}

#[put("/lock", data = "<data>")]
fn put_lock(state: State<Arc<Mutex<Lock>>>, data: Json<LockInput>) -> Result<Status> {
    let lock = &mut state.inner().lock().unwrap();
    let data = data.into_inner();
    if let Some(new_state) = data.state {
        lock.set_state(new_state)?;
    }
    Ok(Status::NoContent)
}

#[post("/lock?toggle")]
fn put_lock_toggle(state: State<Arc<Mutex<Lock>>>) -> Result<Status> {
    let lock = &mut state.inner().lock().unwrap();
    let new_state = match lock.read_state()? {
        LockState::Unlocked => LockState::Locked,
        LockState::Locked => LockState::Unlocked,
    };
    lock.set_state(new_state)?;
    Ok(Status::NoContent)
}

#[get("/lock")]
fn get_lock(state: State<Arc<Mutex<Lock>>>) -> Result<Json<LockOutput>> {
    let lock = &state.inner().lock().unwrap();
    let result = LockOutput {
        state: lock.read_state()?,
        last_change: lock
            .last_change()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs(),
    };
    Ok(Json(result))
}

#[get("/hinge")]
fn get_hinge(state: State<Arc<Mutex<Hinge>>>) -> Result<Json<HingeOutput>> {
    let hinge = &mut state.inner().lock().unwrap();
    let result = HingeOutput {
        state: hinge.read_state()?,
    };
    Ok(Json(result))
}
