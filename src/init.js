export const register = () => {
    jQuery(document).ready(function(){

        "use strict";
        
        // here all ready functions
        
        tm_modalbox();
        tm_nav_bg();
        tm_trigger_menu();
        tm_service_popup();
        tm_modalbox_news();
        tm_modalbox_portfolio();
        progress_by_frenify();
        tm_mycounter();
        tm_projects();
        tm_portfolio();
        tm_cursor();
        tm_imgtosvg();
        tm_popup();
        tm_data_images();
        tm_contact_form();
        tm_owl_carousel();
        tm_input_padding();
        tm_totop();
        tm_down();
        
        jQuery(window).load('body', function(){
            tm_my_load();
        });
        jQuery(window).on('scroll', function(){
            tm_progress_line();
        });
        
    });
}