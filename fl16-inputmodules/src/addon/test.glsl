const float PI = 3.141592653;

void mainImage( out vec4 fragColor, in vec2 fragCoord )
{
    // Normalized pixel coordinates (from 0 to 1)
    vec2 uv = fragCoord/iResolution.xy;
    uv.x -= 0.5;
    uv.x *= 2.0;

    const float width = 1.0 / 9.0;
    const float padding = 0.0;
    const float freq = 0.7;

    float time = iTime * 2.0;
    float shade_coeff = abs(uv.x) + width * 4.0;
    shade_coeff = shade_coeff * shade_coeff * shade_coeff;
    bool left_shaded = int(time / PI - 0.5) % 2 == 0;

    float left = uv.x + sin(uv.y * freq + time) * (1.0 - width - padding);
    left = step(abs(left), width);
    left *= left_shaded ? shade_coeff : 1.0;

    float right = uv.x + sin(uv.y * freq + time + PI) * (1.0 - width - padding);
    right = step(abs(right), width);
    right *= left_shaded ? 1.0 : shade_coeff;

    float d = max(left, right);

    // Output to screen
    fragColor = vec4(d, d, d, 1.0);
}