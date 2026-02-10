const float PI = 3.141592653;

void mainImage( out vec4 fragColor, in vec2 fragCoord )
{
    // Normalized pixel coordinates (from 0 to 1)
    vec2 uv = fragCoord/iResolution.xy;
    uv.x -= 0.5;
    uv.x *= 2.0;
    uv.x *= 2.5;

    const float width = 1.0 / 9.0;
    const float padding = 0.0;
    const float freq = 1.5;

    float time = iTime * 2.0;
    float shade_coeff = abs(uv.x) + width * 2.0;
    shade_coeff = shade_coeff * shade_coeff * shade_coeff;
    float left_offset = sin((uv.y * freq + time) + PI / 2.0) * (1.0 - width - padding);
    bool left_shaded = left_offset > 0.0;

    float left = uv.x + sin(uv.y * freq + time) * (1.0 - width - padding);
    left = smoothstep(width * 1.5, width, abs(left));
    left *= left_shaded ? shade_coeff : 1.0;

    float right = uv.x + sin(uv.y * freq + time + PI) * (1.0 - width - padding);
    right = smoothstep(width * 1.5, width, abs(right));
    right *= left_shaded ? 1.0 : shade_coeff;

    float bar = abs(sin((uv.y * freq + time) * 8.0));
    float bar_mask = sin(uv.x + PI / 2.0) - 0.5;
    float bar_mask_mul = abs(sin(uv.y * freq + time)) * 0.8;
    bar_mask_mul *= bar_mask_mul;
    bar_mask -= (1.0 - bar_mask_mul) * 0.5;
    bar = step(0.8, bar);
    bar_mask = step(0.0, bar_mask);
    bar *= bar_mask;
    bar *= mix(shade_coeff, 1.0, bar_mask_mul * bar_mask_mul * bar_mask_mul);

    float d = max(left, right);
    d = max(d, bar);

    // Output to screen
    fragColor = vec4(d, d, d, 1.0);
}