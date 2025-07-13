#include <stdio.h>
#include <math.h>
#include "pico/stdlib.h"
#include "hardware/gpio.h"
#include "hardware/adc.h"
#include "hardware/pwm.h"

const uint8_t ADC_PIN = 26;
const uint8_t PWM_PIN = 12;
const uint8_t DIR_PIN = 11;

int main() {
    stdio_init_all();

    adc_init();
    adc_gpio_init(ADC_PIN);
    adc_select_input(0);

    static pwm_config pwm_slice_config;
    uint pwm_slice_num;

    gpio_set_function(PWM_PIN, GPIO_FUNC_PWM);
    pwm_slice_num = pwm_gpio_to_slice_num(PWM_PIN);

    pwm_set_clkdiv(pwm_slice_num, 6.103515625);
    pwm_set_wrap(pwm_slice_num, 2047);

    pwm_set_chan_level(pwm_slice_num, PWM_CHAN_A, 0);

    pwm_set_enabled(pwm_slice_num, true);

    gpio_init(DIR_PIN);
    gpio_set_dir(DIR_PIN, GPIO_OUT);

    while (1) {
        int32_t result = (int32_t)adc_read() - 2047;
        printf("Raw value: %5d\r\n", result);

        if (abs(result) < 250) {
            pwm_set_chan_level(pwm_slice_num, PWM_CHAN_A, 0);
            continue;
        }

        pwm_set_chan_level(pwm_slice_num, PWM_CHAN_A, abs(result));

        if (result >= 0) {
            gpio_put(DIR_PIN, 0);
        }
        else {
            gpio_put(DIR_PIN, 1);
        }

        sleep_ms(50);
    }
}
