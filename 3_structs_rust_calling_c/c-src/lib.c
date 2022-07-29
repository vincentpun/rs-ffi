#include <stdio.h>
#include <stdlib.h>

typedef struct {
    float x;
    float y;
} Point;

typedef struct {
    float width;
    float height;
} Size;

typedef struct {
    Point origin;
    Size size;
    const char *description;
} Rect;

float rect_area(Rect *rect) {
    return rect->size.width * rect->size.height;
}

Rect *new_rect(float x, float y, float width, float height, const char *description) {
    Rect *rect = malloc(sizeof(Rect));

    // Not checking if rect is NULL because it won't fail /s.

    Point origin = { x, y };
    Size size = { width, height };

    rect->origin = origin;
    rect->size = size;
    rect->description = description;

    return rect;
}

void free_rect(Rect *rect) {
    free(rect);
}
