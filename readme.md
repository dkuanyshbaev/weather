### Test task.

This is a simple weather forecast application which can show the temperature for the city.\
Data sources: OpenWeather, MetaWeather.

There are two endpoints:\
/day/cityname - for current day data\
/week/cityname - for 5 days data

Example usage:\
curl http://localhost:4444/day/london \
curl http://localhost:4444/week/london

Run with docker:\
docker build -t weather . \
docker run -p 4444:8888 weather
