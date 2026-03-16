# OpenWeather

Project Structure:

Profiles: Stores global variables like API Key, Base URL, and Coordinates (lat/lon).
Object Repository: Contains REST API objects (Get Weather and Req2 Air Polution) with verification scripts.
Test Cases: TC01_VerifyWeatherJakarta script that triggers the API requests and performs validations.
Test Suites: Test collection used to execute the test cases and generate official reports.

Steps to Run:
Open the project in Katalon Studio.
Go to Profiles > default and ensure the apiKey is valid.
Open Test Suites > TS_OpenWeather.
Click the Run button (Green Play icon).

How to Get the Report:
After the test suite execution finishes, go to the Reports folder in the Tests Explorer.
Right-click on the latest report folder.
Select Export as > HTML or PDF. The report will show all successful assertions.
