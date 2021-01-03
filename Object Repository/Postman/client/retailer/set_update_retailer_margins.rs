<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>set_update_retailer_margins</name>
   <tag></tag>
   <elementGuidId>a0320920-2084-45ea-81bd-f7356827bde4</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;[\n  {\n    \&quot;margin\&quot;: \&quot;68.0\&quot;,\n    \&quot;brand\&quot;: {\n      \&quot;id\&quot;: \&quot;27QEtiJJSUCvhR84lxt6nA\&quot;,\n      \&quot;name\&quot;: \&quot;\&quot;\n    },\n    \&quot;retailer\&quot;: {\n      \&quot;id\&quot;: \&quot;yOurNhK8TtW7IKOGHZVT2g\&quot;,\n      \&quot;name\&quot;: \&quot;\&quot;\n    },\n    \&quot;product\&quot;: {\n      \&quot;id\&quot;: \&quot;I0oKeZVxQU2-1NiSLYDCTA\&quot;,\n      \&quot;name\&quot;: \&quot;\&quot;\n    }\n  }\n]&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <katalonVersion>7.8.1</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>${server_url}/retailers/:id/margins</restUrl>
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
      <id>609f36ad-e6c8-49da-bb41-3de3972e9b50</id>
      <masked>false</masked>
      <name>server_url</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
