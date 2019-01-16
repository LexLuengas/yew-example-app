var Twit = require('twit')
const twit_config = require(__dirname + '/twitter_config.json')
var express = require('express')
var cors = require('cors')
var app = express()
const port = 8001

app.use(cors())

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

app.get('/search/:query', function (req, res) {
	if (req.params.query) {
		T.get('search/tweets', {
			q: req.params.query + "+-filter:retweets", // always exclude retweets
			count: 100,
			result_type: "recent",
			lang: "en"
		}, function (err, data, response) {
			res.send({
				"rate_limit_remaining": parseInt(response.headers["x-rate-limit-remaining"]),
				"rate_limit_reset": parseInt(response.headers["x-rate-limit-reset"]),
				"tweets": data.statuses.map(t => {
					return {
						id: t.id_str,
						user_name: t.user.screen_name,
						user_id: t.user.id_str,
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