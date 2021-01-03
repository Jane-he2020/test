<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Add credit card</name>
   <tag></tag>
   <elementGuidId>f5acc288-b2dd-4c27-ba0d-53d17bf839c6</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;cardNumber\&quot;:\&quot;9195e22dd4731a6e14022a1314354f6218f826683ea54a4f5c91aae5b59e5334\&quot;,\n    \&quot;cardCode\&quot;:\&quot;82867ca175df097fe60acc8e1fecd53b\&quot;,\n    \&quot;expirationDate\&quot;:\&quot;128b3b16f437849f2aabbfe3a7d33a79\&quot;,\n    \&quot;isDefault\&quot;:1\n}&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <katalonVersion>7.8.1</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${secure_url}/brands/:id/brandCreditCardInformation</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.secure_url</defaultValue>
      <description></description>
      <id>f900ecca-3ecb-446b-8a67-fe4c21b1853c</id>
      <masked>false</masked>
      <name>secure_url</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
