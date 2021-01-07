```Mermaid
classDiagram
class Vector{
    times(k: number, v: Vector)
    minus(v1: Vector, v2: Vector)$
     plus(v1: Vector, v2: Vector)$
     dot(v1: Vector, v2: Vector)$
     mag(v: Vector)$
     norm(v: Vector)$
     cross(v1: Vector, v2: Vector)
}

class Color {
    constructor(public r: number, public g: number, public b: number) 
     scale(k: number, v: Color) 
     plus(v1: Color, v2: Color)
     times(v1: Color, v2: Color) 
     white = new Color(1.0, 1.0, 1.0);
     grey = new Color(0.5, 0.5, 0.5);
     black = new Color(0.0, 0.0, 0.0);
     background = Color.black;
     defaultColor = Color.black;
     toDrawingColor(c: Color) 
}

class Camera {
    pouet()
}

class Plane {
pouet()
}

class Intersection {
<<Interface>>

}
 
class Ray {
<<Interface>>
}

class Surface {
<<Interface>>
}

class Thing {
<<Interface>>
}

class Scene {
<<Interface>>
}



thing <|-- sphere
thing <|-- Plane




```