/* ----------------------------------------------------------------------------
 * This file was automatically generated by SWIG (http://www.swig.org).
 * Version 4.0.2
 *
 * Do not make changes to this file unless you know what you are doing--modify
 * the SWIG interface file instead.
 * ----------------------------------------------------------------------------- */

package wumpus;

public class wumpus {
  public static SWIGTYPE_p_UserHandle_Engine new_engine(String config_json) {
    long cPtr = wumpusJNI.new_engine(config_json);
    return (cPtr == 0) ? null : new SWIGTYPE_p_UserHandle_Engine(cPtr, false);
  }

  public static void delete_engine(SWIGTYPE_p_UserHandle_Engine data) {
    wumpusJNI.delete_engine(SWIGTYPE_p_UserHandle_Engine.getCPtr(data));
  }

  public static String last_string(SWIGTYPE_p_UserHandle_Engine data) {
    return wumpusJNI.last_string(SWIGTYPE_p_UserHandle_Engine.getCPtr(data));
  }

  public static void execute(SWIGTYPE_p_UserHandle_Engine data, String body) {
    wumpusJNI.execute(SWIGTYPE_p_UserHandle_Engine.getCPtr(data), body);
  }

  public static void handle_event(SWIGTYPE_p_UserHandle_Engine data, String body) {
    wumpusJNI.handle_event(SWIGTYPE_p_UserHandle_Engine.getCPtr(data), body);
  }

  public static String initial_html(SWIGTYPE_p_UserHandle_Engine data) {
    return wumpusJNI.initial_html(SWIGTYPE_p_UserHandle_Engine.getCPtr(data));
  }

  public static boolean is_shutdown_required(SWIGTYPE_p_UserHandle_Engine data) {
    return wumpusJNI.is_shutdown_required(SWIGTYPE_p_UserHandle_Engine.getCPtr(data));
  }

  public static String last_response_json(SWIGTYPE_p_UserHandle_Engine data) {
    return wumpusJNI.last_response_json(SWIGTYPE_p_UserHandle_Engine.getCPtr(data));
  }

}
