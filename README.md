# PVGIS scraper

Scrape timeseries data from PVGIS in json format

### Under the hood

1.  Generate random latitude and longitude
1.  Check that generated values are on land and in database range; if not, go back to step one
1.  Query the API with the random generated coordinates
1.  Write response to disk
1.  Repeat steps one to four n times, n is specified by the user
