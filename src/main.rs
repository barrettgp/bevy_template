use bevy::prelude::*;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, tick)
        .insert_resource(Custom::new())
        .run();
}



fn setup(
    mut commands: Commands,
) {

}


fn tick() {

}


#[derive(Resource)]
struct Custom(f32);


impl Custom {
    fn new() -> Self {
        return Custom(1.)
    }
}
