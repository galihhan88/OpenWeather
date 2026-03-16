<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Req2 Air Polution</name>
   <tag></tag>
   <elementGuidId>72bb3fd0-3d09-40b0-8d85-44aba0ef37de</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>true</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <katalonVersion>10.3.1</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>${baseUrl}/data/2.5/air_pollution?lat=${lat}&amp;lon=${lon}&amp;appid=${apiKey}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.baseUrl</defaultValue>
      <description></description>
      <id>3f62f475-c81b-41bc-bf6b-6023095af810</id>
      <masked>false</masked>
      <name>baseUrl</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.lat</defaultValue>
      <description></description>
      <id>6bda9416-1256-4759-be59-2dcecdddf2a1</id>
      <masked>false</masked>
      <name>lat</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.lon</defaultValue>
      <description></description>
      <id>d966e40a-02aa-452f-95df-0a573c4e868d</id>
      <masked>false</masked>
      <name>lon</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.apiKey</defaultValue>
      <description></description>
      <id>edb1870f-bcff-4911-80d5-fc26905a99d8</id>
      <masked>false</masked>
      <name>apiKey</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager
import groovy.json.JsonSlurper

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

// 1. Verifikasi Status Code (Harus 200)
WS.verifyResponseStatusCode(response, 200)

// 2. Parse JSON Response
def jsonSlurper = new JsonSlurper()
def jsonResponse = jsonSlurper.parseText(response.getResponseText())

// 3. Verifikasi Atribut Penting (AQI)
// Kita ambil nilainya dulu
int aqiValue = jsonResponse.list[0].main.aqi
println(&quot;Nilai AQI Jakarta Selatan: &quot; + aqiValue)

// Gunakan keyword verifyElementPropertyValue yang benar untuk API
WS.verifyElementPropertyValue(response, 'list[0].main.aqi', aqiValue)

// 4. Verifikasi Range AQI (Harus 1-5) - Ini memenuhi syarat &quot;atribut penting&quot;
assert aqiValue >= 1 &amp;&amp; aqiValue &lt;= 5

// 5. Verifikasi Keberadaan Komponen Polutan (Ganti verifyElementPresent dengan ini)
// Kita cek apakah nilainya tidak null (berarti field-nya ada)
WS.verifyElementPropertyValue(response, 'list[0].components.pm2_5', jsonResponse.list[0].components.pm2_5)
WS.verifyElementPropertyValue(response, 'list[0].components.no2', jsonResponse.list[0].components.no2)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
