//! Provides Unity style button interactions
//!
//! Currently supports color tinting and image swapping.
//!
//! # Getting started
//! Import the crate to bring all necessary types into scope:
//! ```
//! use bevy_button_transitions::*;
//! ```
//!
//! Add [`ButtonTransitionsPlugin`] to setup the system:
//! ```
//! # /*
//! app.add_plugins(ButtonTransitionsPlugin);
//! # */
//! ```
//!
//! Add the [`ButtonTransition`] component to your buttons
//! ```
//! commands.spawn((
//!     ButtonTransition::ColorTint(ColorTint::default()),
//!     ImageNode {
//!         image: asset_server.load("normal_image.png"),
//!         ..default()
//!     },
//!     Node {
//!         width: Val::Px(250.0),
//!         height: Val::Px(80.0),
//!         ..default()
//!     },
//! ));
//! ```
//!
//! Be aware that the color tint button transition needs an image to tint!
use bevy_asset::Handle;

use bevy_app::Plugin;
use bevy_app::PostUpdate;

use bevy_color::Color;
use bevy_color::palettes::css::WHITE;

use bevy_derive::Deref;
use bevy_derive::DerefMut;

use bevy_ecs::component::Component;
#[cfg(feature = "bevy_reflect")]
use bevy_ecs::reflect::ReflectComponent;
use bevy_ecs::schedule::IntoSystemConfigs;
use bevy_ecs::system::Query;

use bevy_image::Image;

use bevy_ui::Interaction;
use bevy_ui::widget::Button;
use bevy_ui::widget::ImageNode;

/// A [`Plugin`] that sets up button transitions.
pub struct ButtonTransitionsPlugin;

impl Plugin for ButtonTransitionsPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        #[cfg(feature = "bevy_reflect")]
        app.register_type::<ButtonTransition>();
        #[cfg(feature = "bevy_reflect")]
        app.register_type::<Interactable>();
        app.add_systems(
            PostUpdate,
            (update_button_interactions, update_interactables).chain(),
        );
    }
}

/// A [`Component`] that automatically adds transitions for a button.
///
/// # Remarks
/// Remember to add an ImageNode with a base image for the ColorTint Transition - otherwise
/// there is nothing to tint!
#[derive(Component)]
#[require(Button, Interactable, ImageNode)]
#[cfg_attr(
    feature = "bevy_reflect",
    derive(bevy_reflect::Reflect),
    reflect(Component)
)]
pub enum ButtonTransition {
    ColorTint(ColorTint),
    ImageSwap(ImageSwap),
}

/// Defines the different tints of a [`ButtonTransition::ColorTint`].
#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
pub struct ColorTint {
    pub normal_color: Color,
    pub hovered_color: Color,
    pub pressed_color: Color,
    pub disabled_color: Color,
}

impl Default for ColorTint {
    fn default() -> Self {
        ColorTint {
            normal_color: WHITE.into(),
            hovered_color: Color::srgba(0.9607843, 0.9607843, 0.9607843, 1.0),
            pressed_color: Color::srgba(0.7843137, 0.7843137, 0.7843137, 1.0),
            disabled_color: Color::srgba(0.7843137, 0.7843137, 0.7843137, 0.5019608),
        }
    }
}

/// Defines the different image swaps of a [`ButtonTransition::ImageSwap`].
#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
pub struct ImageSwap {
    pub normal_image: Handle<Image>,
    pub hovered_image: Handle<Image>,
    pub pressed_image: Handle<Image>,
    pub disabled_image: Handle<Image>,
}

/// A [`Component`] that determines if a button can be pressed.
///
/// Will automatically be added with the ButtonTransition but has to
/// be queried by a user manually if a button is pressed.
/// # Example
/// ```
/// fn check_button_press(query: Query<(&Interaction, &Interactable)>) {
///     for (interaction, interactable) in &query {
///         if interactable.0 == false {
///             continue;
///         }
///         match interaction {
///             _ => todo!("add interactions here")
///         }
///     }
/// }
/// ```
#[derive(Component, Deref, DerefMut)]
#[cfg_attr(
    feature = "bevy_reflect",
    derive(bevy_reflect::Reflect),
    reflect(Component)
)]
pub struct Interactable(pub bool);

impl Default for Interactable {
    fn default() -> Self {
        Interactable(true)
    }
}

fn update_button_interactions(mut query: Query<(&mut ImageNode, &ButtonTransition, &Interaction)>) {
    for (mut image_node, transition, interaction) in &mut query {
        match transition {
            ButtonTransition::ColorTint(tint) => {
                let color = match interaction {
                    Interaction::Hovered => tint.hovered_color,
                    Interaction::Pressed => tint.pressed_color,
                    Interaction::None => tint.normal_color,
                };
                image_node.color = color;
            }
            ButtonTransition::ImageSwap(swap) => {
                let image = match interaction {
                    Interaction::Hovered => swap.hovered_image.clone(),
                    Interaction::Pressed => swap.pressed_image.clone(),
                    Interaction::None => swap.normal_image.clone(),
                };
                image_node.image = image;
            }
        }
    }
}

fn update_interactables(mut query: Query<(&mut ImageNode, &Interactable, &ButtonTransition)>) {
    for (mut image_node, interactable, button_transition) in &mut query {
        if interactable.0 == false {
            match button_transition {
                ButtonTransition::ColorTint(color) => image_node.color = color.disabled_color,
                ButtonTransition::ImageSwap(swap) => image_node.image = swap.disabled_image.clone(),
            }
        }
    }
}
