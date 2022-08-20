use crate::color::Color;
use crate::ray::Intersection;
use crate::object::light::Light;
use crate::object::Shade;

pub enum Material {
    Diffuse {color: Color}
}

impl Shade for Material {
    fn get_color(&self, intersection: &Intersection, lights: &[Light]) -> Color
    {
        let mut radiance = Color::new(0.0);
        if let Intersection::Hit{normal,..} = intersection {

            lights.iter().for_each(|light|{
                match light {
                    Light::Ambient{..} => {},
                    Light::Directional{dir, color: light_color} => {
                        let intensity = f32::max(normal * dir, 0.0);
                        radiance = radiance + match self {
                            Material::Diffuse{color: material_color} =>  intensity * light_color * material_color
                        } 
                    },
                    Light::Point{pos, dir, color} => {},
                }
            });
        }
        radiance
    }
}