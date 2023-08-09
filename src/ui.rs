

use bevy::{
    prelude::*,
    sprite::MaterialMesh2dBundle,
    window::{Window, PrimaryWindow},
    ecs::system::EntityCommands};

use crate::backend::*;

mod backend;

#[derive(Component)]
struct Deck(Vec<Card>);

fn main() {
        App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup))
        .run()

}

#[derive(Component)]
struct PlayedCard(Card);

#[derive(Component)]
struct PlayArea;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let mut deck = make_shuffled_deck();

    commands.spawn(Camera2dBundle::default());

    let window = window_query.get_single().unwrap();

    let height = window.height();
    let width = window.width();

    display_cards(&mut commands, &mut meshes, &mut materials, deck.drain(deck.len()-12..), &(0., 0.), &(width*0.8, height*0.8));

    commands.spawn(Deck(deck));

}

const CARD_LENGTH: f32 = 100.;
const CARD_WIDTH: f32 = 60.;

fn display_cards(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    cards: &[Card],
    &(centerx, centery): &(f32, f32),
    &(sizex, sizey): &(f32, f32),
) {
    // Figure out what the shape is gonna be
    let num_cards = cards.len();

    let cols = num_cards/3;

    // Cards should be equally spaced from each other and from the edges of the given space
    // Divide space into n+1 equal spaces
    let ystep = sizey/4.;
    let ystart = centery - sizey/2. + ystep;

    let xstep = sizex/((cols+1) as f32);
    let xstart = centerx - sizex/2. + xstep;
    
    for (num, card) in cards.enumerate() {
	spawn_card(commands, meshes, materials, card, &(xstart + xstep * ((num/3) as f32), ystart + ystep * ((num%3) as f32)), &(CARD_LENGTH, CARD_WIDTH)); 
    }
}

fn spawn_card(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    card: &Card,
    &(centerx, centery): &(f32, f32),
    &(sizex, sizey): &(f32, f32),
) {
    let mut cardbase = commands.spawn((PlayedCard(*card), MaterialMesh2dBundle {
	    mesh: meshes
		.add(shape::Quad::new(Vec2::new(1., 1.)).into())
		.into(),
	    material: materials.add(ColorMaterial::from(Color::WHITE)),
	transform: Transform::from_translation(Vec3::new(centerx, centery, 0.))
	    .with_scale(Vec3::new(sizex, sizey, 1.)),
	..default()}));

    let cardfn = match card.shape {
	Shape::Oval => draw_oval,
	Shape::Wave => draw_wave,
	Shape::Diamond => draw_diamond,
    };

    let shape_width = 0.2;
    let shape_spacing = shape_width*1.2;
    let shape_height = 0.8;

    let shape_dimensions = (shape_width, shape_height);

    let material = match card.colour {
	Colour::Green => materials.add(ColorMaterial::from(Color::GREEN)),
	Colour::Red => materials.add(ColorMaterial::from(Color::RED)),
	Colour::Purple => materials.add(ColorMaterial::from(Color::PURPLE)),
    };

    match card.number {
	Number::One => {
	    cardfn(meshes, &mut cardbase, material.clone(), &(0., 0.), &shape_dimensions);
	}
	Number::Two => {
	    cardfn(meshes, &mut cardbase, material.clone(), &(0. - shape_spacing/2., 0.), &shape_dimensions);
	    cardfn(meshes, &mut cardbase, material.clone(), &(0. + shape_spacing/2., 0.), &shape_dimensions);
	}
	Number::Three => {
	    cardfn(meshes, &mut cardbase, material.clone(), &(0. - shape_spacing, 0.), &shape_dimensions);
	    cardfn(meshes, &mut cardbase, material.clone(), &(0., 0.), &shape_dimensions);
	    cardfn(meshes, &mut cardbase, material.clone(), &(0. + shape_spacing, 0.), &shape_dimensions);
	}
    }
}

fn draw_oval(
    meshes: &mut ResMut<Assets<Mesh>>,
    cardbase: &mut EntityCommands,
    material: Handle<ColorMaterial>,
    &(centerx, centery): &(f32, f32),
    &(sizex, sizey): &(f32, f32),
) {
    cardbase.with_children(|child_builder|{
	child_builder.spawn(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(0.5).into()).into(),
            material,
            transform: Transform::from_translation(Vec3::new(centerx, centery, 0.1))
		.with_scale(Vec3::new(sizex, sizey, 1.)),
            ..default()
	});
    });
}

fn draw_wave(
    meshes: &mut ResMut<Assets<Mesh>>,
    cardbase: &mut EntityCommands,
    material: Handle<ColorMaterial>,
    &(centerx, centery): &(f32, f32),
    &(sizex, sizey): &(f32, f32),
) {}
fn draw_diamond(
    meshes: &mut ResMut<Assets<Mesh>>,
    cardbase: &mut EntityCommands,
    material: Handle<ColorMaterial>,
    &(centerx, centery): &(f32, f32),
    &(sizex, sizey): &(f32, f32),
) {}


fn setups(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    // Circle
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(50.).into()).into(),
        material: materials.add(ColorMaterial::from(Color::PURPLE)),
        transform: Transform::from_translation(Vec3::new(-150., 0., 0.)),
        ..default()
    });

    // Rectangle
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.25, 0.25, 0.75),
            custom_size: Some(Vec2::new(50.0, 100.0)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(-50., 0., 0.)),
        ..default()
    });

    // Quad
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes
            .add(shape::Quad::new(Vec2::new(50., 100.)).into())
            .into(),
        material: materials.add(ColorMaterial::from(Color::LIME_GREEN)),
        transform: Transform::from_translation(Vec3::new(50., 0., 0.)),
        ..default()
    });

    // Hexagon
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::RegularPolygon::new(50., 6).into()).into(),
        material: materials.add(ColorMaterial::from(Color::TURQUOISE)),
        transform: Transform::from_translation(Vec3::new(150., 0., 0.)),
        ..default()
    });
}
