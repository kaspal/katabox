import posixpath


def clean_path(path: str) -> str:
    # Obviously correct: the standard library's path normaliser (after collapsing the
    # leading slashes, which POSIX treats specially when there are exactly two).
    return posixpath.normpath("/" + path.lstrip("/"))
