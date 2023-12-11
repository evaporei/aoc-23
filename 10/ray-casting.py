def point_inside_polygon(x, y, poly):
    """ Deciding if a point is inside (True, False otherwise) a polygon,
    where poly is a list of pairs (x,y) containing the polygon's vertices.
    The algorithm is called the 'Ray Casting Method' """
    n = len(poly)
    inside = False
    p1x, p1y = poly[0]
    for i in range(n):
        p2x, p2y = poly[i % n]
        if y > min(p1y, p2y):
            if y <= max(p1y, p2y):
                if x <= max(p1x, p2x):
                    if p1y != p2y:
                        xinters = (y-p1y) * (p2x-p1x) / (p2y-p1y) + p1x
                    if p1x == p2x or x <= xinters:
                        inside = not inside
        p1x, p1y = p2x, p2y
    return inside
