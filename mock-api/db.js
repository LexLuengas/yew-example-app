var Twit = require('twit')
const twit_config = require(__dirname + '/twitter_config.json')
var express = require('express')
var app = express()
const port = 8001

var T = new Twit(Object.assign(twit_config, {
	timeout_ms: 60 * 1000,
	strictSSL: true,
}))

T.get('account/verify_credentials', {
	include_entities: false,
	skip_status: true,
	include_email: false
}, (err, res) => {
	if (err) throw err;
	console.log('[Server] Twitter authentication successful.\r\n')
})

app.get('/test', (req, res) => {
	res.send({'message': 'Success!'})
})

app.get('/keyword/:keyword', function (req, res) {
	if (req.params.keyword) {
		T.get('search/tweets', {
			q: req.params.keyword,
			count: 100
		}, function (err, data, response) {
			res.send({
				"rate-limit-remaining": response.headers["x-rate-limit-remaining"],
				"rate-limit-reset": response.headers["x-rate-limit-reset"],
				"tweets": data.statuses.map(t => {
					return {
						id: t.id,
						user_name: t.user.screen_name,
						user_id: t.user.id,
						text: t.text
					}
				})
			})
		})
	}
})

let server = app.listen(port, () => console.log(`[Server] App listening on port ${port}`))

server.on('error', function(e) {
    console.log(e);
    console.log("Port already in use!");
});