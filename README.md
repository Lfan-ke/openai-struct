# openai-api
[openai api](https://github.com/openai/openai-openapi/blob/master/openapi.yaml) on rust serde

api阅读可使用[api-hub](https://app.swaggerhub.com/)在线阅读openapi.yaml

目前实现的版本：[api-hub](https://app.swaggerhub.com/apis/none-080/OpenAPI-RS/0.0.1)

中文可参考：[OpenAICTO.com](https://www.openaicto.com/api-reference)

根据api的框架划分项目框架

```shell
npm install @openapitools/openapi-generator-cli -g

openapi-generator-cli validate -i openapi.yaml

openapi-generator-cli generate -i openapi.yaml -g rust -o ./rust-client --skip-validate-spec
openapi-generator-cli generate -i openapi.yaml -g rust-server -o ./rust-server --skip-validate-spec
```

> todo: 创建`dev`分支，以及完善`CI`
