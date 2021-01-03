<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>set_a_retailer's_addresses</name>
   <tag></tag>
   <elementGuidId>fe5a9643-7515-4f88-9817-9d9b4b2b57c3</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;[ \n    { \&quot;label\&quot;: \&quot;Warehouse\&quot;, \n      \&quot;addressee\&quot;: \&quot;Attn: Receiving\&quot;, \n      \&quot;streetAddressLine_1\&quot;: \&quot;766 Brackbill Road\&quot;, \n      \&quot;streetAddressLine_2\&quot;: \&quot;Suite 1285\&quot;, \n      \&quot;city\&quot;: \&quot;Philadelphia\&quot;, \n      \&quot;region\&quot;: \&quot;PA\&quot;, \n      \&quot;postalCode\&quot;: \&quot;17527\&quot;, \n      \&quot;country\&quot;: { \n          \&quot;id\&quot;: \&quot;us\&quot; \n          },\n      \&quot;hq\&quot;: true, \n      \&quot;defaultShipping\&quot;: false \n    } \n]&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <katalonVersion>7.8.1</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>${server_url}/retailers/:id/addresses</restUrl>
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
      <id>de811368-a778-41e5-80d8-bba7d7b74fb3</id>
      <masked>false</masked>
      <name>server_url</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
