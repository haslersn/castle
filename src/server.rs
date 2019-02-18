use crate::hinge::Hinge;
use crate::hinge::HingeState;
use crate::lock::Lock;
use crate::lock::LockState;
use crate::util::Result;
use rocket::config::Config;
use rocket::config::Environment;
use rocket::State;
use rocket_contrib::json::Json;
use std::sync::Arc;
use std::sync::Mutex;

#[derive(Deserialize)]
pub struct ServerSettings {
    pub mount_point: String,
    pub port: u16,
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
        routes![
            put_lock_state,
            toggle_lock_state,
            get_lock_state,
            get_hinge_state
        ],
    )
    .manage(hinge)
    .manage(lock)
    .launch();
}

#[put("/lock-state", data = "<new_state>")]
fn put_lock_state(
    state: State<Arc<Mutex<Lock>>>,
    new_state: Json<LockState>,
) -> Result<Json<LockState>> {
    let lock = &mut state.inner().lock().unwrap();
    let new_state = new_state.into_inner();
    lock.set_state(new_state)?;
    Ok(Json(new_state))
}

#[post("/lock-state?toggle")]
fn toggle_lock_state(state: State<Arc<Mutex<Lock>>>) -> Result<Json<LockState>> {
    let lock = &mut state.inner().lock().unwrap();
    let new_state = match lock.read_state()? {
        LockState::Unlocked => LockState::Locked,
        LockState::Locked => LockState::Unlocked,
    };
    lock.set_state(new_state)?;
    Ok(Json(new_state))
}

#[get("/lock-state")]
fn get_lock_state(state: State<Arc<Mutex<Lock>>>) -> Result<Json<LockState>> {
    let lock = &state.inner().lock().unwrap();
    Ok(Json(lock.read_state()?))
}

#[get("/hinge-state")]
fn get_hinge_state(state: State<Arc<Mutex<Hinge>>>) -> Result<Json<HingeState>> {
    let hinge = &mut state.inner().lock().unwrap();
    Ok(Json(hinge.read_state()?))
}
