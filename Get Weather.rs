<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Get Weather</name>
   <tag></tag>
   <elementGuidId>66fd6124-243a-4c12-81ea-574c177dd0af</elementGuidId>
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
   <restUrl>${baseUrl}/data/2.5/forecast?lat=${lat}&amp;lon=${lon}&amp;appid=${apiKey}&amp;units=metric</restUrl>
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
      <id>949c73a0-f54d-4ea5-a28c-23d8ef75d4dd</id>
      <masked>false</masked>
      <name>baseUrl</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.lat</defaultValue>
      <description></description>
      <id>a6757cdf-a489-4e5a-8617-35409b1e4889</id>
      <masked>false</masked>
      <name>lat</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.lon</defaultValue>
      <description></description>
      <id>4133488e-9e64-49c7-847a-0f9f46226a7c</id>
      <masked>false</masked>
      <name>lon</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.apiKey</defaultValue>
      <description></description>
      <id>9203323a-ef96-4fac-a67a-9b54189fff2f</id>
      <masked>false</masked>
      <name>apiKey</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

// 1. Verifikasi Status Code
WS.verifyResponseStatusCode(response, 200)

// 3. Verifikasi Isi Body (Jakarta Selatan terdeteksi sebagai Jakarta di OpenWeather)
WS.verifyElementPropertyValue(response, 'city.name', 'Rawa Barat')

// 4. Verifikasi Atribut Penting (Contoh: koordinat)
WS.verifyElementPropertyValue(response, 'city.coord.lat', -6.2615)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
