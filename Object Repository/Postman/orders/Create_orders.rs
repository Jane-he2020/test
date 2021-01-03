<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Create_orders</name>
   <tag></tag>
   <elementGuidId>9f404ab9-6380-41aa-8a9c-f2b409e3a5fa</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n\&quot;retailerReferenceId\&quot;: \&quot;ABC123\&quot;, \n\&quot;orderDate\&quot;: \&quot;2019-10-22\&quot;, \n\&quot;shipDate\&quot;: \&quot;2019-10-29\&quot;, \n\&quot;cancelDate\&quot;: \&quot;2019-11-05\&quot;, \n\&quot;shipToAddress\&quot;: {\n\&quot;addressee\&quot;: \&quot;John Doe\&quot;, \n\&quot;streetAddressLine_1\&quot;: \&quot;766 Brackbill Road\&quot;, \n\&quot;streetAddressLine_2\&quot;: \&quot;Suite 1285\&quot;,\n\&quot;city\&quot;: \&quot;Philadelphia\&quot;,\n\&quot;region\&quot;: \&quot;PA\&quot;,\n\&quot;postalCode\&quot;: \&quot;17527\&quot;,\n\&quot;country\&quot;: {\n\&quot;id\&quot;: \&quot;us\&quot; \n},\n\&quot;lines\&quot;: [ \n    {\n\&quot;product\&quot;: {\n\&quot;id\&quot;: \&quot;39X8EoAWRQyQZvG9SqOrhg\&quot;\n}, \n\&quot;orderType\&quot;: {\n\&quot;id\&quot;: \&quot;purchase\&quot;\n },\n\&quot;quantity\&quot;: \&quot;500\&quot; \n}\n] \n}\n}&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <katalonVersion>7.8.1</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${server_url}/orders</restUrl>
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
      <id>39c018a3-2902-4152-9869-8c945a5f46e1</id>
      <masked>false</masked>
      <name>server_url</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
