from aiohttp import web

routes = web.RouteTableDef()

@routes.get('/f/{number}')
async def hello(request):
    n = int(request.match_info['number'])
    resp = { 'result': str(n*2) }
    return web.json_response(resp)

app = web.Application()
app.add_routes(routes)
web.run_app(app)
