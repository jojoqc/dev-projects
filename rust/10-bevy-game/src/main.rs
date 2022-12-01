use bevy::prelude::*;
//use bevy::input::InputPlugin;

#[derive(Component)]
struct Position { x: f32, y: f32 }
#[derive(Component)]
struct Person;
#[derive(Component)]
struct Name(String);

pub struct HelloPlugin;

impl Plugin for HelloPlugin{
    fn build(&self, app: &mut App){
        println!("{:?}",3+3);
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
        .add_startup_system(add_people)
        .add_system(greet_people);
    }
}

fn print_position_system(query: Query<&Transform>) {
    for transform in query.iter() {
        println!("position: {:?}", transform.translation);
    }
}

fn hello_world() {
    println!("hello world!");
}

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Elaina Proctor".to_string())));
    commands.spawn((Person, Name("Renzo Hume".to_string())));
    commands.spawn((Person, Name("Zayna Nieves".to_string())));
}

#[derive(Resource)]
struct GreetTimer(Timer);

fn greet_people(time:Res<Time>, mut timer:ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    if timer.0.tick(time.delta()).just_finished(){
        for name in query.iter() {
            println!("hello {}!", name.0);
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(HelloPlugin)
        .run();
}
