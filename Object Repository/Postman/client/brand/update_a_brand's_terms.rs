<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>update_a_brand's_terms</name>
   <tag></tag>
   <elementGuidId>2369e83e-cfab-4b36-b33d-c660a3a50565</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n     \&quot;paymentTerm\&quot;: { \n         \&quot;id\&quot;: \&quot;net90\&quot;, \n         \&quot;name\&quot;: \&quot;Net 90\&quot; \n         },\n    \&quot;landingCommission\&quot;: \&quot;3.45\&quot;, \n    \&quot;usesLandingFulfillment\&quot;: false, \n    \&quot;minimumOrderAmountUsd\&quot;: 1000, \n    \&quot;minimumOrderQuantity\&quot;: 10 \n    }&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <katalonVersion>7.8.1</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${server_url}/brands/:id/terms</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.server_url</defaultValue>
      <description></description>
      <id>e7b17319-92d9-4341-8a76-a5fc923261ec</id>
      <masked>false</masked>
      <name>server_url</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
