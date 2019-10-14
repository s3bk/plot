precision mediump float;

uniform float uLineWidth;
uniform vec4 uColor;
uniform float uBlendFactor; //1.5..2.5
varying vec2 vLineCenter;
void main(void)
{
      vec4 col = uColor;        
      float d = length(vLineCenter-gl_FragCoord.xy);
      float w = uLineWidth;
      if (d>w)
        col.w = 0.;
      else
        col.w *= pow(float((w-d)/w), uBlendFactor);
      gl_FragColor = uColor; //col;
}
