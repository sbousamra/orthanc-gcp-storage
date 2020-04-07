#include "OrthancCPlugin.h"

int OrthancPluginCheckVersionWrapped(OrthancPluginContext* context) {
  return OrthancPluginCheckVersion(context);
}

void OrthancPluginLogErrorWrapped(OrthancPluginContext* context, const char* message) {
  return OrthancPluginLogError(context, message);
}

void OrthancPluginLogWarningWrapped(OrthancPluginContext* context, const char* message) {
  return OrthancPluginLogWarning(context, message);
}

void OrthancPluginLogInfoWrapped(OrthancPluginContext* context, const char* message) {
  return OrthancPluginLogInfo(context, message);
}

void OrthancPluginSetDescriptionWrapped(OrthancPluginContext*  context, const char* description) {
  return OrthancPluginSetDescription(context, description);
}

char *OrthancPluginGetConfigurationWrapped(OrthancPluginContext* context) {
  return OrthancPluginGetConfiguration(context);
}

void OrthancPluginRegisterStorageAreaWrapped(
  OrthancPluginContext* context, 
  OrthancPluginStorageCreate  create,
  OrthancPluginStorageRead    read,
  OrthancPluginStorageRemove  remove
) {
  return OrthancPluginRegisterStorageArea(context, create, read, remove);
}