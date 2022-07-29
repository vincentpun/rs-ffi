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

extern float rect_area(Rect *rect);
extern Rect *new_rect(float x, float y, float width, float height, const char *description);
extern void free_rect(Rect *rect);

int main(void) {
    // The client code is responsible for calling new and free.
    Rect *rect = new_rect(10.0, 10.0, 50.0, 100.0, "My rectangle!");
    printf("Area = %f\n", rect_area(rect));
    printf("x = %f\n", rect->origin.x);
    printf("y = %f\n", rect->origin.y);
    printf("Description = %s\n", rect->description);
    free_rect(rect);

    return 0;
}
