<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>update_a_retailer's_terms</name>
   <tag></tag>
   <elementGuidId>43a25562-039e-4d4b-aec1-7b3b4d67fa71</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\r\n    \&quot;paymentTerm\&quot;: {\r\n        \&quot;id\&quot;: \&quot;net60\&quot;\r\n    },\r\n    \&quot;margin\&quot;: \&quot;75\&quot;,\r\n    \&quot;marketingCoOp\&quot;: \&quot;10\&quot;,\r\n    \&quot;damagesBudget\&quot;: \&quot;5\&quot;,\r\n    \&quot;usDomesticShippingObligation\&quot;: \&quot;brand\&quot;,\r\n    \&quot;testersObligation\&quot;: \&quot;brand\&quot;,\r\n    \&quot;fixturesObligation\&quot;: \&quot;brand\&quot;,\r\n    \&quot;signageObligation\&quot;: \&quot;brand\&quot;,\r\n    \&quot;acceptsOverseasFreight\&quot;: true,\r\n    \&quot;productTicketingRequired\&quot;: true,\r\n    \&quot;ediRequired\&quot;: true,\r\n    \&quot;routingGuideRequired\&quot;: true\r\n}&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <katalonVersion>7.8.1</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${server_url}/retailers/:id/terms?_skipNulls=false</restUrl>
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
      <id>258e8471-483a-44b7-aa46-e0aa151a5796</id>
      <masked>false</masked>
      <name>server_url</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
