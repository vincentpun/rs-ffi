#include <stdio.h>
#include <stdlib.h>
#include "main.h"

int main(void) {
    // The client code is responsible for calling new and free.
    RectType type = {
        Rounded,
        { .rounded = 3.0 },
    };

    Rect *rect = new_rect(10.0, 10.0, 50.0, 100.0, "My rectangle!", type);
    printf("Area = %f\n", rect_area(rect));
    printf("x = %f\n", rect->origin.x);
    printf("y = %f\n", rect->origin.y);
    printf("Description = %s\n", rect->description);
    printf("Type = %s\n", rect_type(rect));
    free_rect(rect);

    return 0;
}
