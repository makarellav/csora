#include <stdio.h>
#include <stdbool.h>
#include <SDL2/SDL.h>
#include <stdint.h>
#include <math.h>

#define max(a, b) \
    ({ __typeof__ (a) _a = (a); \
       __typeof__ (b) _b = (b); \
       _a > _b ? _a : _b; })

#define min(a, b) \
    ({ __typeof__ (a) _a = (a); \
       __typeof__ (b) _b = (b); \
       _a < _b ? _a : _b; })

SDL_Window *window = NULL;
SDL_Renderer *renderer = NULL;
SDL_Texture *texture_buffer = NULL;

int window_width = 800;
int window_height = 600;

bool is_running = false;

bool init(void) {
    if (SDL_Init(SDL_INIT_EVERYTHING) != 0) {
        fprintf(stderr, "Failed to initialize SDL: %s\n", SDL_GetError());

        return false;
    }

    SDL_DisplayMode display_mode;
    if (SDL_GetDesktopDisplayMode(0, &display_mode) == 0) {
        window_width = display_mode.w;
        window_height = display_mode.h;
    }

    window = SDL_CreateWindow(
        "Software Rasterizer", 
        SDL_WINDOWPOS_CENTERED, 
        SDL_WINDOWPOS_CENTERED, 
        window_width, 
        window_height, 
        SDL_WINDOW_BORDERLESS
    );

    if (!window) {
        fprintf(stderr, "Failed to create an SDL window: %s\n", SDL_GetError());

        return false;
    }

    renderer = SDL_CreateRenderer(
        window,
        -1,
        0
    );

    if (!renderer) {
        fprintf(stderr, "Failed to create an SDL renderer %s\n", SDL_GetError());

        return false;
    }

    SDL_SetWindowFullscreen(window, SDL_WINDOW_FULLSCREEN);

    return true;
}

void setup(void) {
    texture_buffer = SDL_CreateTexture(
        renderer,
        SDL_PIXELFORMAT_ARGB8888,
        SDL_TEXTUREACCESS_STREAMING,
        window_width,
        window_height
    );
}

void process_input(void) {
    SDL_Event event;
    SDL_PollEvent(&event);

    switch (event.type) {
        case SDL_QUIT:
            is_running = false;
            break;
        case SDL_KEYDOWN:
            if (event.key.keysym.sym == SDLK_ESCAPE) {
                is_running = false;
            }
            break;
    }
}

void update(void) {

}

void clear_buffer(uint32_t *buffer, uint32_t color) {
    for (int i = 0; i < window_width * window_height; i++) {
        buffer[i] = color;
    }
}

void draw_grid(uint32_t *buffer, uint32_t color) {
    for (int y = 0; y < window_height; y++) {
        for (int x = 0; x < window_width; x++) {
            if (y % 50 == 0 || x % 50 == 0) {
                buffer[(window_width * y) + x] = color;
            }
        }
    }
}

void draw_rect(
    uint32_t *buffer, 
    uint32_t color, 
    int x_start, 
    int y_start,
    int w,
    int h
) {
    int y_bound = min(y_start + h, window_height);
    int x_bound = min(x_start + w, window_width);

    int y_start_clamped = max(y_start, 0);
    int x_start_clamped = max(x_start, 0);

    for (int y = y_start_clamped; y < y_bound; y++) {
        for (int x = x_start_clamped; x < x_bound; x++) {
            buffer[(window_width * y) + x] = color;
        }
    }
}

void render_buffer(void) {
    int row_size = (int)(sizeof(uint32_t) * window_width);
    void *locked_buffer = NULL;

    if (
        SDL_LockTexture(
            texture_buffer, 
            NULL, 
            &locked_buffer, 
            &row_size
        ) != 0
    ) {
        fprintf(stderr, "Failed to lock texture: %s\n", SDL_GetError());

        return;
    }

    uint32_t *buffer = (uint32_t *)locked_buffer;

    clear_buffer(buffer, 0xFF000000);
    draw_grid(buffer, 0xFFFF0000);
    draw_rect(buffer, 0xFF0000FF, 100, 100, 500, 500);

    SDL_UnlockTexture(texture_buffer);

    SDL_RenderCopy(
        renderer,
        texture_buffer,
        NULL,
        NULL
    );
}

void render(void) {
    render_buffer();

    SDL_RenderPresent(renderer);
}

void release_resources(void) {
    SDL_DestroyRenderer(renderer);
    SDL_DestroyWindow(window);
    SDL_Quit();
}

int main(void) {
    is_running = init();

    setup();

    while (is_running) {
        process_input();
        update();
        render();
    }

    release_resources();

    return 0;
}