check_first_load();

on('body', 'click', '.a_1', function() {
  is = this;
  if (!is.classList.contains("active")){
    is.nextElementSibling.classList.remove("active");is.nextElementSibling.classList.add("pointer")
    is.classList.add("active"); is.classList.remove("pointer")
    nav = is.parentElement.parentElement;
    tabs = nav.nextElementSibling;
    tab_item = tabs.querySelector(".auth_tab_2");
    tab_item.classList.remove("active", "in");
    cur = tabs.querySelector(".auth_tab_1");
    cur.classList.add("active", "in")
}
});

on('body', 'click', '.a_2', function() {
  is = this;
  if (!is.classList.contains("active")){
    is.previousElementSibling.classList.remove("active");is.previousElementSibling.classList.add("pointer")
    is.classList.add("active"); is.classList.remove("pointer")
    nav = is.parentElement.parentElement;
    tabs = nav.nextElementSibling;
    tab_item = tabs.querySelector(".auth_tab_1");
    tab_item.classList.remove("active", "in");
    cur = tabs.querySelector(".auth_tab_2");
    cur.classList.add("active", "in")
}
});

on('body', 'click', '.apps_btn', function() {
  toggle_nav_first_span();
});
on('body', 'click', '.pages_btn', function() {
  toggle_nav_second_span();
});

on('body', 'click', '.window_fullscreen_hide', function() {
  mob_menu_hide()
});
on('body', 'click', '.mob_menu', function() {
  this.style.display = "none";
  document.querySelector(".window_fullscreen").style.display = "block";
});

on('body', 'input', '.mobile_folder_search', function() {
    _this = this;
    value = _this.value;
    if (value == "") {
      _this.parentElement.parentElement.parentElement.querySelector(".search_result").innerHTML= "";
      return;
    }
    parent = _this.parentElement.parentElement.parentElement;
    search_block = parent.querySelector(".search_result");
    if (value == "") {
      search_block.innerHTML= "";
      return;
    }
    else if (value.length < 3) {
      search_block.innerHTML= "";
      return;
    }

    if (_this.getAttribute("data-folder")) {
      folder = _this.getAttribute("data-folder")
    } else {
      folder = ""
    };
    url = "/search" + folder + "/" + _this.value + "/";

    var ajax_link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
    ajax_link.open( 'GET', url + "?ajax=1", true );
    ajax_link.setRequestHeader('X-Requested-With', 'XMLHttpRequest');
    ajax_link.onreadystatechange = function () {
      if ( this.readyState == 4 && this.status == 200 ) {
        elem_ = document.createElement('span');
        elem_.innerHTML = ajax_link.responseText;
        elem_.querySelector(".is_paginate") ?
        (
          search_section = elem_.querySelector(".is_paginate"),
          search_block.innerHTML = search_section.innerHTML.replaceAll(new RegExp(value, 'ig'), "<span class='selected'>" + value + "</span>")
        ) : search_block.innerHTML = "<div style='margin-top: 40px;'><div class='align-center'><span class='border' style='padding: 10px 15px;'>Искать пока не из чего...</div></div>";
        content_block.classList.add("hidden");
      }
    }
    ajax_link.send();
});
