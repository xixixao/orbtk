/// Used to define a property.
#[macro_export]
macro_rules! property {
    ($(#[$property_doc:meta])* $property:ident($type:ty)) => {
        use dces::prelude::{Entity, EntityComponentManager};
        use crate::widget::{PropertySource, get_property};

        #[derive(Default, Debug, Clone, PartialEq)]
        $(#[$property_doc])*
        pub struct $property(pub $type);
        
        impl $property {
            /// Returns the value of a property.
            pub fn get(entity: Entity, ecm: &EntityComponentManager) -> $type {
                get_property::<$property>(entity, ecm).0
            }
        }

         impl Into<PropertySource<$property>> for $property {
            fn into(self) -> PropertySource<$property> {
                PropertySource::Value(self)
            }
        }

        impl From<$property> for $type {
            fn from(property: $property) -> $type {
                property.0.into()
            }
        }

        impl From<$type> for $property {
            fn from(value: $type) -> $property {
                $property(value)
            }
        }

        impl Into<PropertySource<$property>> for $type {
            fn into(self) -> PropertySource<$property> {
                PropertySource::Value($property::from(self))
            }
        }

        impl Into<PropertySource<$property>> for Entity {
            fn into(self) -> PropertySource<$property> {
                PropertySource::Source(self)
            }
        }
    };
}

/// Used to define a widget, with properties and event handlers.
#[macro_export]
macro_rules! widget {
    ( $(#[$widget_doc:meta])* $widget:ident $(<$state:ident>)* $(: $( $handler:ident ),*)* $( { $($(#[$prop_doc:meta])* $property:ident: $property_type:tt ),* } )* ) => {
        use std::{ any::TypeId, rc::Rc, collections::HashMap};

        use dces::prelude::{Component, ComponentBox, SharedComponentBox };

        use crate::{event::EventHandler,
            properties::{Bounds, Constraint, VerticalAlignment, HorizontalAlignment, Visibility, Name},
            widget::{PropertySource, Widget, BuildContext},
            structs::Point};

        $(#[$widget_doc])*
        pub struct $widget {
            attached_properties: HashMap<TypeId, ComponentBox>,
            shared_attached_properties: HashMap<TypeId, SharedComponentBox>,
            event_handlers: Vec<Rc<dyn EventHandler>>,
            bounds: Bounds,
            constraint: Constraint,
            name: Option<Name>,
            horizontal_alignment: HorizontalAlignment,
            vertical_alignment: VerticalAlignment,
            margin: Margin,
            enabled: Enabled,
            visibility: Visibility,
             $(
                $(
                    $property: Option<PropertySource<$property_type>>,
                )*
             )*
            children: Vec<Entity>,
        }

        impl $widget {
            /// Sets or shares an attached property.
            pub fn attach<P: Component>(mut self, property: impl Into<PropertySource<P>>) -> Self {
                match property.into() {
                    PropertySource::Value(value) => {
                        self.attached_properties.insert(TypeId::of::<P>(), ComponentBox::new(value));
                    },
                    PropertySource::Source(source) => {
                        self.shared_attached_properties.insert(TypeId::of::<P>(), SharedComponentBox::new(TypeId::of::<P>(), source));
                    }
                }
                self
            }

             /// Sets or shares the constraint property.
            pub fn constraint<P: Into<PropertySource<Constraint>>>(self, constraint: P) -> Self {
                self.attach(constraint)
            }

            /// Sets or shares the vertical alignment property.
            pub fn vertical_alignment<P: Into<PropertySource<VerticalAlignment>>>(self, vertical_alignment: P) -> Self {
                self.attach(vertical_alignment)
            }

            /// Sets or shares the horizontal alignment property.
            pub fn horizontal_alignment<P: Into<PropertySource<HorizontalAlignment>>>(self, horizontal_alignment: P) -> Self {
                self.attach(horizontal_alignment)
            }

            /// Sets or shares the visibility property.
            pub fn visibility<P: Into<PropertySource<Visibility>>>(self, visibility: P) -> Self {
                self.attach(visibility)
            }

            /// Sets or shares the margin property.
            pub fn margin<P: Into<PropertySource<Margin>>>(self, margin: P) -> Self {
                self.attach(margin)
            }

            /// Sets or shares the enabled property.
            pub fn enabled<P: Into<PropertySource<Enabled>>>(self, enabled: P) -> Self {
                self.attach(enabled)
            }

            /// Inserts a new width.
            pub fn width(mut self, width: f64) -> Self {
                self.constraint.set_width(width);
                self
            }

            /// Inserts a new height.
            pub fn height(mut self, height: f64) -> Self {
                self.constraint.set_height(height);
                self
            }

            /// Inserts a new size.
            pub fn size(mut self, width: f64, height: f64) -> Self {
                self.constraint.set_width(width);
                self.constraint.set_height(height);
                self
            }

            /// Inserts a new min_width.
            pub fn min_width(mut self, min_width: f64) -> Self {
                self.constraint.set_min_width(min_width);
                self
            }

            /// Inserts a new min_height.
            pub fn min_height(mut self, min_height: f64) -> Self {
                self.constraint.set_min_height(min_height);
                self
            }

            /// Inserts a new min_size.
            pub fn min_size(mut self, min_width: f64, min_height: f64) -> Self {
                self.constraint.set_min_width(min_width);
                self.constraint.set_min_height(min_height);
                self
            }

            /// Inserts a new max_width.
            pub fn max_width(mut self, max_width: f64) -> Self {
                self.constraint.set_max_width(max_width);
                self
            }

            /// Inserts a new max_height.
            pub fn max_height(mut self, max_height: f64) -> Self {
                self.constraint.set_max_height(max_height);
                self
            }

            /// Inserts a new min_size.
            pub fn max_size(mut self, max_width: f64, max_height: f64) -> Self {
                self.constraint.set_max_width(max_width);
                self.constraint.set_max_height(max_height);
                self
            }

            /// Sets the debug name of the widget.
            pub fn name<P: Into<Name>>(mut self, name: P) -> Self {
                self.name = Some(name.into());
                self
            }

            $(
                $(
                    $(#[$prop_doc])*
                    pub fn $property<P: Into<PropertySource<$property_type>>>(mut self, $property: P) -> Self {
                        if !self.$property.is_none() {
                            return self;
                        }

                        self.$property = Some($property.into());
                        self
                    }
                )*
            )*
        }

        $(
            $(
                impl $handler for $widget {}
            )*
        )*

        impl Widget for $widget {
            fn create() -> Self {
                $widget {
                    attached_properties: HashMap::new(),
                    shared_attached_properties: HashMap::new(),
                    event_handlers: vec![],
                    bounds: Bounds::default(),
                    constraint: Constraint::default(),
                    name: None,
                    horizontal_alignment: HorizontalAlignment::default(),
                    vertical_alignment: VerticalAlignment::default(),
                    visibility: Visibility::default(),
                    margin: Margin::default(),
                    enabled: Enabled(false),
                    $(
                        $(
                            $property: None,
                        )*
                    )*
                    children: vec![],
                }
            }

            fn insert_handler(mut self, handler: impl Into<Rc<dyn EventHandler>>) -> Self {
                self.event_handlers.push(handler.into());
                self
            }

            fn child(mut self, child: Entity) -> Self {
                self.children.push(child);
                self
            }

            $(
                fn state(self) -> Option<Rc<State>> {
                    Rc::new($state::new())
                }
            )*

            fn build(self, context: &mut BuildContext) -> Entity {
                let entity = context.create_entity();

                let this = self.template(entity, context);

                if let Some(render_object) = this.render_object() {
                    context.register_render_object(entity, render_object);
                }

                context.register_layout(entity, this.layout());

                // register default set of properties
                context.register_property(entity, this.bounds);
                context.register_property(entity, this.constraint);
                context.register_property(entity, this.vertical_alignment);
                context.register_property(entity, this.horizontal_alignment);
                context.register_property(entity, this.visibility);
                context.register_property(entity, this.margin);

                // register helpers
                context.register_property(entity, Point::default());
                
                // register attached properties
                for (_, property) in this.attached_properties {
                    context.register_property_box(entity, property);
                }

                for (_, property) in this.shared_attached_properties {
                    context.register_property_shared_box(entity, property);
                }

                // register properties
                $(
                    $(
                        if let Some($property) = this.$property {
                            match $property {
                                PropertySource::Value(value) => {
                                    context.register_property(entity, value);
                                },
                                PropertySource::Source(source) => {
                                    context.register_shared_property::<$property_type>(entity, source);
                                }
                            }
                        }
                    )*
                )*

                // register event handlers
                for handler in this.event_handlers {
                    context.register_handler(entity, handler);
                }

                // register name
                if let Some(name) = this.name {
                    println!("{} (id = {}, children_len = {})", name.0, entity, this.children.len());
                    context.register_property(entity, name);                   
                }

                for child in this.children {
                    context.append_child(entity, child);
                }

                entity
            }
        }
    };
}
