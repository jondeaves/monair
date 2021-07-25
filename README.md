# Monair

Utility to monitor and report on air quality. It uses data from [Air Quality Programmatic API](https://aqicn.org/api/) to show parse relevant data.

## Configuration

The app will read configuration from a `.env` file locally or from environment variables in production. See example in `.example.env`.

### API Key

The API Key can be obtained from [this page](https://aqicn.org/data-platform/token/)

### Lat and Long

The `LAT` and `LONG` values will be used to find data from the nearest station to provided location.