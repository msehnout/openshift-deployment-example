from aiohttp import web
import logging
import aiohttp

routes = web.RouteTableDef()

@routes.get('/')
async def hello(request):
    request.app['counter'] += 1
    n = request.app['counter']
    return web.Response(text="Hello, world {}".format(n))


@routes.get('/f')
async def f(request):
    status, response = 0, ''
    async with aiohttp.ClientSession() as session:
        async with session.get('http://httpbin.org/get') as resp:
            status = resp.status
            response = await resp.text()
    return web.Response(text="{},\n{}".format(status, response))


app = web.Application()
app.add_routes(routes)
app['counter'] = 0
logging.basicConfig(level=logging.DEBUG)
web.run_app(app)
