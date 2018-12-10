uniform vec4 scale;
uniform float time;

attribute vec2 position;
attribute vec2 velocity;

void
main ()
{
        vec2 pos = ((position + velocity * time) + scale.zw) * scale.xy;
        gl_Position = vec4(pos, 0.0, 1.0);
}

//@@

void
main ()
{
        gl_FragColor = vec4(1.0);
}
