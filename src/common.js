module.exports.scoreTemperature = function(temp_f) {
  return scoreHeat(temp_f) + scoreCold(temp_f);
};

function scoreHeat(temp_f) {
  return Math.max(0, temp_f - 75);
}

function scoreCold(temp_f) {
  return Math.max(0, 68 - temp_f);
}
