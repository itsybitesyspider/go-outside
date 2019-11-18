const lib = require("./index.js");

module.exports.handler = async function(event, _context, callback) {
  const body = JSON.parse(event.body);
  const periods = await lib.list(body.lat, body.lon);

  const now = lib.get_now(periods);
  const best = lib.get_best(periods);

  const response = {
    answer: now===best,
    best: lib.human(best)
  };

  callback(null, {
    statusCode: 200,
    headers: {
      "Access-Control-Allow-Origin": "*"
    },
    body: JSON.stringify(response),
  });
};
