def largest_tile(width: int, height: int) -> int:
    while height:
        width, height = height, width % height
    return width
