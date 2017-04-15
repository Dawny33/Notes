import socket


class Resolver:
    def __init__(self):
        self._cache = {}

    # Resolves DNS normally if not found in cache,
    # and then adds to cache.
    def __call__(self, host):
        if host not in self._cache:
            self._cache[host] = socket.gethostbyname(host)
        return self._cache[host]

    # Clears the cache
    def clear(self):
        self._cache.clear()

    # Tells whether the dns/host is in cache or not
    def has_host(self, host):
        return host in self._cache
