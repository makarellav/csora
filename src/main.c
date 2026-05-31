#include <stdio.h>
#include <stdbool.h>
#include <SDL2/SDL.h>

SDL_Window *window;
SDL_Renderer *renderer;

bool init(void) {
    if (SDL_Init(SDL_INIT_EVERYTHING) != 0) {
        fprintf(stderr, "Failed to initialize SDL: %s\n", SDL_GetError());

        return false;
    }

    window = SDL_CreateWindow(
        NULL, 
        SDL_WINDOWPOS_CENTERED, 
        SDL_WINDOWPOS_CENTERED, 
        800, 
        600, 
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
        fprintf(stderr, "Failed to create an SDL renderer", SDL_GetError());

        return false;
    }

    return true;
}

int main(void) {
    bool is_running = init();

    return 0;
}