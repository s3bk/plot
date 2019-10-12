precision mediump float;

attribute vec4 aVertexPosition; 
uniform mat4 uTransform; 
uniform vec2 uViewPort; //Width and Height of the viewport
varying vec2 vLineCenter;
void main(void)
{
    vec4 pp = uTransform * aVertexPosition;
    gl_Position = pp;
    vec2 vp = uViewPort;
    vLineCenter = 0.5*(pp.xy + vec2(1, 1))*vp;
}