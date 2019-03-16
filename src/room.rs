#[derive(Deserialize)]
pub struct Room {
  pub name: String,
  pub lights: Vec<String>,
  pub state: RoomState,
}

#[derive(Deserialize)]
pub struct RoomState {
  pub all_on: bool,
  pub any_on: bool,
}
