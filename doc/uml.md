title Raytracer

main->cam:get_ray(u, v)
activate cam
cam->*ray:«create»
main<--cam:ray
deactivate cam

main->ray:ray_color(&world, depth)
activate ray

participantgroup #e0eeee world
participant world
participant objects[i]
end

opt depth = 0
main<--ray: BLACK
end

ray->world:hit_anything(ray, t_min, t_max)
activate world
loop i < world.objects.len()

world->objects[i]:hit(ray, t_min, t_max)
activate objects[i]

participantgroup #c1cdcd hit_record
participant material
participant record.ray
end

objects[i]->*material:«create»
objects[i]->*record.ray:«create»

world<--objects[i]:Option<hit_record>
deactivate objects[i]

end
ray<--world:Option<hit_record>
deactivate world

opt Some(hit_record)
ray->material:scatter(ray, &hit_record)
activate material
ray<--material: (ray, color)
deactivate material

ray->record.ray:ray_color(&world, depth - 1)
activate record.ray
note over record.ray:Rekursive diagramme sind nicht\nnötig, die Funktion wird deshalb hier\nnicht weiter ausgeführt
ray<--record.ray:color
deactivate record.ray
main<--ray: color
end

main<--ray: BACKGROUND
deactivate ray
