static event: LambdaEvent = LambdaEvent { 
  payload: ApiGatewayProxyRequest { 
    resource: Some("/temperature"), 
    path: Some("/temperature"), 
    http_method: POST, 
    headers: {}, 
    multi_value_headers: {}, 
    query_string_parameters: QueryMap({}), 
    multi_value_query_string_parameters: QueryMap({}), 
    path_parameters: {}, 
    stage_variables: {}, 
    request_context: ApiGatewayProxyRequestContext { 
      account_id: Some("058264156666"), 
      resource_id: Some("s5uu73"), 
      operation_name: None, 
      stage: Some("test-invoke-stage"), 
      domain_name: Some("testPrefix.testDomainName"), 
      domain_prefix: Some("testPrefix"), 
      request_id: Some("40d22bf7-ba70-4533-ab48-e8cef1742c39"), 
      protocol: Some("HTTP/1.1"), 
      identity: ApiGatewayRequestIdentity { 
        cognito_identity_pool_id: None, 
        account_id: Some("058264156666"), 
        cognito_identity_id: None, 
        caller: Some("058264156666"), 
        api_key: Some("test-invoke-api-key"), 
        api_key_id: Some("test-invoke-api-key-id"), 
        access_key: Some("ASIAQ3EGQPH5BNJ5KM6Y"), 
        source_ip: Some("test-invoke-source-ip"), 
        cognito_authentication_type: None, 
        cognito_authentication_provider: None, 
        user_arn: Some("arn:aws:iam::058264156666:root"), 
        user_agent: Some("Mozilla/5.0 (X11; Linux x86_64; rv:126.0) Gecko/20100101 Firefox/126.0"), user: Some("058264156666") 
      }, 
      resource_path: Some("/temperature"), 
      path: Some("/temperature"), 
      authorizer: ApiGatewayRequestAuthorizer { 
        jwt: None, 
        fields: {}, 
        iam: None }, 
        http_method: POST, 
        request_time: Some("04/Jun/2024:22:26:19 +0000"), 
        request_time_epoch: 1717539979108, 
        apiid: Some("ghsk08xg9b") 
      }, 
      body: None, 
      is_base64_encoded: false }, 
      context: Context 
      { 
        request_id: "ce345bd1-b4a8-45a0-81d5-74e411d92599", 
        deadline: 1717539987951, 
        invoked_function_arn: "arn:aws:lambda:us-west-1:058264156666:function:temperature_lambda", 
        xray_trace_id: Some("Root=1-665f948b-eba2d555a34f103e528307f0;Parent=3fa6de7047a4d924;Sampled=0;Lineage=5bc03bde:0"), 
        client_context: None, 
        identity: None, 
        env_config: Config { 
          function_name: "temperature_lambda", 
          memory: 128, 
          version: "$LATEST", 
          log_stream: "2024/06/04/[$LATEST]20a26613b0234042816f2a321fd7b5f2", 
          log_group: "/aws/lambda/temperature_lambda" 
        } 
      } 
    };
