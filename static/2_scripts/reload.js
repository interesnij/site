function on(elSelector, eventName, selector, fn) {var element = document.querySelector(elSelector);element.addEventListener(eventName, function(event) {var possibleTargets = element.querySelectorAll(selector);var target = event.target;for (var i = 0, l = possibleTargets.length; i < l; i++) {var el = target;var p = possibleTargets[i];while (el && el !== element) {if (el === p) {return fn.call(p, event);}el = el.parentNode;}}});};

function loadScripts( src ) {
    var script = document.createElement("SCRIPT"),
        head = document.getElementsByTagName( "head" )[ 0 ],
        span = document.getElementsByTagName( "span" )[ 0 ],
        error = false;
    script.type = "text/javascript";
    script.onload = script.onreadystatechange = function( e ){
        if ( ( !this.readyState || this.readyState == "loaded" || this.readyState == "complete" ) ) {
            if ( !error ) {
                removeListeners();
            } else {
                null
            }
        }
    };
    script.onerror = function() {
        error = true;
        removeListeners();
    }
    function errorHandle( msg, url, line ) {
        if ( url == src ) {
            error = true;
            removeListeners();
        }
        return false;
    }
    function removeListeners() {
        script.onreadystatechange = script.onload = script.onerror = null;

        if ( window.removeEventListener ) {
            window.removeEventListener('error', errorHandle, false );
        } else {
            window.detachEvent("onerror", errorHandle );
        }
    }
    if ( window.addEventListener ) {
        window.addEventListener('error', errorHandle, false );
    } else {
        window.attachEvent("onerror", errorHandle );
    }
    script.src = src;
    head.appendChild( script );
    //span.appendChild( script );
}; 

function getCookie(name) {
    const cookies = document.cookie.split(';');
    for (let i = 0; i < cookies.length; i++) {
        let c = cookies[i].trim().split('=');
        if (c[0] === name) {
            return c[1];
        }
    }
    return "";
}
function setCookie(name, value, days) {
    let cookie = `${name}=${encodeURIComponent(value)}`;
    if (days) {
        const expiry = new Date();
        expiry.setDate(expiry.getDate() + days);
        cookie += `; expires=${expiry.toUTCString()}`;
    }
    document.cookie = cookie + "; path=/";
};

function get_background_color() {
  background = getCookie("background");
  if (background == "dark_wood" || background == "dark") {
    document.body.classList.add("v-dark");
  }
}

function check_first_load() {
  loc = window.location.href;
  if (loc.indexOf('template') > -1) {
    url = loc + "&ajax=1"; 
  }
  else {
    url = loc + "?ajax=1"; 
  }
    get_background_color();
    ajax_link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
    ajax_link.open( 'GET', url, true );
    ajax_link.setRequestHeader('X-Requested-With', 'XMLHttpRequest');
    ajax_link.onreadystatechange = function () {
        if ( this.readyState == 4 && this.status == 200 ) {
            elem_ = document.createElement('span');
            elem_.innerHTML = ajax_link.responseText;
            document.body.innerHTML = elem_.innerHTML;
            loadScripts('/static/1_scripts/progressive-image.js?ver1');
            mouseCirMove(); 
            reloadAjax();
            window.history.pushState ({"url":url}, document.title, url);
        }
    }
    ajax_link.send();
  };

  function ajax_get_reload(url, history_enable) {
    var ajax_link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
      ajax_link.open( 'GET', url + "?ajax=2", true );
      ajax_link.setRequestHeader('X-Requested-With', 'XMLHttpRequest');
      ajax_link.onreadystatechange = function () {
        if ( this.readyState == 4 && this.status == 200 ) {
          rtr = document.querySelector('.ajax_1');
          // статистика
          $link = document.location.pathname;
          meta_block = rtr.querySelector(".doc_title");
          if (meta_block.getAttribute("data-id")) {
            $object_id = meta_block.getAttribute("data-id");
          }
          else {
            $object_id = ""
          }
          $page_id = meta_block.getAttribute("page-id");
          $title = meta_block.getAttribute("data-title");
          // 
          elem_ = document.createElement('span');
          elem_.innerHTML = ajax_link.responseText;
          //sidebar = elem_.querySelector(".sidebar");
  
          rtr.innerHTML = elem_.innerHTML;
  
          _meta = rtr.querySelector(".doc_title");
          _title = _meta.getAttribute("data-title");
          _uri = "https://вебсервисы.рф" + _meta.getAttribute("data-uri");
          _description = _meta.getAttribute("data-description");
          _image = "https://вебсервисы.рф" + _meta.getAttribute("data-image");
          document.title = _title;
          document.querySelector('meta[name="url"]').setAttribute("content", _uri);
          document.querySelector('meta[name="title"]').setAttribute("content", _title);
          document.querySelector('meta[name="description"]').setAttribute("content", _description);
          document.querySelector('meta[name="image"]').setAttribute("content", _image);
          document.querySelector('link[rel="canonical"]').setAttribute("href", _uri);
  
          window.scrollTo(0,0);
          if (history_enable) {
            window.history.pushState ({"url":url}, $title, url);
          }
          reloadAjax();
          //get_active_button();
          //get_page_view_time(120);
          //scrolled(rtr);
          //get_stat_meta($link, $title, $object_id, $page_id);
          //get_document_opacity_1();
        }
      }
      ajax_link.send();
  };

  window.addEventListener('popstate', function (e) {
    ajax_get_reload(history.state["url"], false);
    //return false
  })

  on('body', 'click', '.ajax', function(event) {
    event.preventDefault();
    ajax_get_reload(this.getAttribute("href"), true)
  });

  async function reloadAjax( $off ) {
    await dsnGrid.destoryBuild();
    //await loadData( "poster" );
    //await loadData( "src" );
    //await loadData( "srcset" );
    if ( !$off ) {
        window.$effectScroll = await effectScroller();
        window.$animate = await effectAnimate();
        await navMenu().init();
        await dsnGrid.removeWhiteSpace( ".site-header ul.extend-container li > a" );
        await changeStyle();
    }
    if ( $off ) {
        await mouseCirMove( $off );
    }
    $( "a.vid" ).YouTubePopUp();
    await ContactModel();
    await $effectScroll.start();
    await effctStickyNavBar();
    await $animate.allInt();
    await slider().run();
    await Isotope();
    await projectSlider().run();
    await accordion();
    await linkRightPaginate();
    await magnificPopup();
    await justifiedGallery();
    await hoverReveal();
    await contactValidator();
    await dsnAjax().ajaxLoad();
    await dropHash();
    await $( ".twentytwenty" ).twentytwenty();
}

  function hoverReveal() {
    const getMousePos = ( e ) => {
        var posx = 0;
        var posy = 0;
        if ( !e ) e = window.event;
        if ( e.pageX || e.pageY ) {
            posx = e.pageX;
            posy = e.pageY;
        } else if ( e.clientX || e.clientY ) {
            posx = e.clientX + document.body.scrollLeft + document.documentElement.scrollLeft;
            posy = e.clientY + document.body.scrollTop + document.documentElement.scrollTop;
        }
        return {
            x: posx,
            y: posy
        }
    };
    const getRandomFloat = ( min, max ) => ( Math.random() * ( max - min ) + min ).toFixed( 2 );
    class HoverImgFx13 {
        constructor( el ) {
            this.DOM = { el: el };
            this.DOM.reveal = document.createElement( 'div' );
            this.DOM.reveal.className = 'hover-reveal';
            this.DOM.reveal.innerHTML = `<div class="hover-reveal__img" style="background-image:url(${ this.DOM.el.dataset.img })"></div>`;
            this.DOM.el.appendChild( this.DOM.reveal );
            this.DOM.revealImg = this.DOM.reveal.querySelector( '.hover-reveal__img' );
            dsnGrid.convertTextLine( this.DOM.el.querySelectorAll( '.work__item-text' ) )
            this.DOM.letters = [ ...this.DOM.el.querySelectorAll( '.work__item-text span' ) ];
            this.initEvents();
        }
        initEvents() {
            this.positionElement = ( ev ) => {
                if ( $body.hasClass( 'dsn-ajax-effect' ) ) {
                    return;
                }
                const mousePos = getMousePos( ev );
                if ( $effectScroll.isScroller() ) {
                    const scrollbar = $effectScroll.getScrollbar();
                    this.DOM.reveal.style.top = `${ mousePos.y - ( this.DOM.reveal.offsetHeight / 2 ) + ( scrollbar.offset.y ) }px`;
                    this.DOM.reveal.style.left = `${ mousePos.x - ( this.DOM.reveal.offsetHeight / 2 - 60 ) - scrollbar.offset.x }px`;
                } else {
                    const docScrolls = {
                        left: document.body.scrollLeft + document.documentElement.scrollLeft,
                        top: document.body.scrollTop + document.documentElement.scrollTop
                    };
                    this.DOM.reveal.style.top = `${ mousePos.y + 20 - ( docScrolls.top / 150 ) }px`;
                    this.DOM.reveal.style.left = `${ mousePos.x + 20 - docScrolls.left }px`
                }
            };
            this.mouseenterFn = ( ev ) => {
                if ( $body.hasClass( 'dsn-ajax-effect' ) ) return;
                this.positionElement( ev );
                this.animateLetters();
                this.showImage();
            };
            this.mousemoveFn = ev => requestAnimationFrame( () => {
                if ( $body.hasClass( 'dsn-ajax-effect' ) ) return;

                this.positionElement( ev );
            } );
            this.mouseleaveFn = () => {
                if ( $body.hasClass( 'dsn-ajax-effect' ) ) return;
                this.hideImage();
            };
            this.DOM.el.addEventListener( 'mouseenter', this.mouseenterFn );
            this.DOM.el.addEventListener( 'mousemove', this.mousemoveFn );
            this.DOM.el.addEventListener( 'mouseleave', this.mouseleaveFn );
        }
        showImage() {
            TweenMax.killTweensOf( this.DOM.revealImg );
            this.tl = new TimelineMax( {
                onStart: () => {
                    this.DOM.reveal.style.opacity = 1;
                    TweenMax.set( this.DOM.el, { zIndex: 1000 } );
                }
            } )
                .add( 'begin' )
                .set( this.DOM.revealImg, { transformOrigin: '95% 50%', x: '100%' } )
                .add( new TweenMax( this.DOM.revealImg, 0.2, {
                    ease: Sine.easeOut,
                    startAt: { scaleX: 0.5, scaleY: 1 },
                    scaleX: 1.5,
                    scaleY: 0.7
                } ), 'begin' )
                .add( new TweenMax( this.DOM.revealImg, 0.8, {
                    ease: Expo.easeOut,
                    startAt: { rotation: 10, y: '5%', opacity: 0 },
                    rotation: 0,
                    y: '0%',
                    opacity: 1
                } ), 'begin' )
                .set( this.DOM.revealImg, { transformOrigin: '0% 50%' } )
                .add( new TweenMax( this.DOM.revealImg, 0.6, {
                    ease: Expo.easeOut,
                    scaleX: 1,
                    scaleY: 1,
                    opacity: 1
                } ), 'begin+=0.2' )
                .add( new TweenMax( this.DOM.revealImg, 0.6, {
                    ease: Expo.easeOut,
                    x: '0%'
                } ), 'begin+=0.2' )
        }
        hideImage() {
            TweenMax.killTweensOf( this.DOM.revealImg );
            this.tl = new TimelineMax( {
                onStart: () => {
                    TweenMax.set( this.DOM.el, { zIndex: 999 } );
                },
                onComplete: () => {
                    TweenMax.set( this.DOM.el, { zIndex: '' } );
                    TweenMax.set( this.DOM.reveal, { opacity: 0 } );
                }
            } )
                .add( 'begin' )
                .add( new TweenMax( this.DOM.revealImg, 0.2, {
                    ease: Sine.easeOut,
                    opacity: 0,
                    x: '-20%'
                } ), 'begin' );
        }
        animateLetters() {
            TweenMax.killTweensOf( this.DOM.letters );
            this.DOM.letters.forEach( letter => TweenMax.set( letter, {
                y: Math.round( Math.random() ) === 0 ? '50%' : '0%',
                opacity: 0
            } ) );
            TweenMax.to( this.DOM.letters, 1, {
                ease: Expo.easeOut,
                y: '0%',
                opacity: 1
            } );
        }
    }
    Array.from( document.querySelectorAll( "[data-fx=\"1\"] > .work__item a, a[data-fx=\"1\"]" ) ).forEach( link => new HoverImgFx13( link ) );


}
function magnificPopup() {
    let option = {
        delegate: "a:not(.effect-ajax)",
        type: "image",
        closeOnContentClick: false,
        closeBtnInside: false,
        mainClass: "mfp-with-zoom", // this class is for CSS animation below
        gallery: {
            enabled: true,
        },
        zoom: {
            enabled: true,
            duration: 400, // don't foget to change the duration also in CSS
            easing: "cubic-bezier(0.36, 0, 0.66, -0.56)", // CSS transition easing function
            opener: function ( element ) {
                return element.find( "img" );
            },
        },
        callbacks: {
            open: function () {
                $( "html" ).css( { margin: 0 } );
            }
        },
    };
    $( ".gallery-portfolio" ).each( function () {
        $( this ).magnificPopup( option );
    } );

    if ( $( ".has-popup .pop-up" ).length ) option.delegate = "a.pop-up";
    $( ".has-popup" ).magnificPopup( option );

}
function justifiedGallery() {
    $( ".gallery-portfolio" ).each( function () {
        $( this ).justifiedGallery( {
            rowHeight: 250,
            margins: 15,
        } );
    } );
}
function effectBackForward() {
    $wind.on( "popstate", function ( e ) {
        if ( window.location.hash.length ) {
            $wind.scrollTop( 0 );
            dsnGrid.scrollTop( window.location.hash, 1, -100 );
            return;
        }
        if ( document.location.href.indexOf( "#" ) > -1 ) {
            return;
        }
        setTimeout( function () {
            dsnAjax().backAnimate( document.location );
        }, 100 );
    } );
}
function ContactModel( $off ) {
    const btn = $( '.contact-btn' );
    if ( $off )
        btn.off( 'click' );
    btn.on( 'click', () => {
        $body.toggleClass( 'dsn-show-contact' );
    } )
}
function effectScroller() {
    const Scrollbar = window.Scrollbar;
    const locked_scroll = "locked-scroll";
    var myScrollbar = document.querySelector( "#dsn-scrollbar" );
    return {
        start: function () {
            $body.removeClass( locked_scroll );
            $( '.box-view-item .box-img .dsn-scroll-box' ).each( function () {
                Scrollbar.init( this, {
                    damping: 0.06,
                } );
            } );
            if ( !this.isScroller( true ) ) return;
            Scrollbar.init( myScrollbar, {
                damping: 0.06,
                renderByPixels: true,
                continuousScrolling: false,
                plugins: {
                    overscroll: true
                },
            } );
            this.contactForm();
        },
        contactForm: function () {
            const form = $( '.contact-modal .contact-container' );
            if ( form.length )
                Scrollbar.init( form.get( 0 ), {
                    damping: 0.06,
                } );
        },
        isScroller: function ( $print ) {
            if ( $print )
                myScrollbar = document.querySelector( "#dsn-scrollbar" );
            let hasSc = !$body.hasClass( "dsn-effect-scroll" ) || dsnGrid.isMobile() || myScrollbar === null;
            if ( hasSc && $print ) {
                $body.addClass( "dsn-mobile" );
            }
            return !hasSc;
        },
        locked: function () {
            $body.addClass( locked_scroll );
            if ( this.isScroller() ) {
                let scroll = this.getScrollbar();
                if ( scroll !== undefined ) {
                    scroll.destroy();
                }
                scroll = null;
            }
        },
        getScrollbar: function ( $id ) {
            if ( $id === undefined ) {
                return Scrollbar.get( myScrollbar );
            }
            return Scrollbar.get( document.querySelector( $id ) );
        },
        getListener: function ( $obj, $useWin = true ) {
            if ( $obj === undefined ) return;
            let $this = this;
            if ( $this.isScroller() ) {
                $this.getScrollbar().addListener( $obj );
            } else {
                if ( $useWin )
                    $wind.on( "scroll", $obj );
            }
            $this = null;
        },
        scrollNavigate: function () {
            let top = $( ".wrapper" ).offset();
            top = top ? top.top : 0;
            $( ".scroll-top , .scroll-to-top" ).on( "click", function () {
                dsnGrid.scrollTop( 0, 2 );
            } );

            $( ".scroll-d" ).on( "click", function () {
                dsnGrid.scrollTop( top, 2,
                    ( $( ".scrollmagic-pin-spacer" ).height() * -1 ) - 200 || -200 );
            } );

        },
    };
}

  function effectAnimate() {
    const easeAnimate = Linear.easeNone;
    return {
        allInt: function () {
            this.clearControl()
                .then( () => {
                    this.headerPages();
                } )
                .then( () => {
                    this.animations();
                } )
                .then( () => {
                    this.parallaxMulti();
                } )
                .then( () => {
                    this.parallaxImg();
                } )
                .then( () => {
                    this.moveSection();
                } )
                .then( () => {
                    this.parallaxImgHover();
                } )
                .then( () => {
                    this.nextProject();
                } )
                .then( () => {
                    this.dsnScrollTop();
                } )
                .then( () => {
                    this.translateSection();
                } )
                .then( () => {
                    $effectScroll.scrollNavigate();

                    $effectScroll.getListener( function ( status ) {
                        $scene.forEach( function ( scene ) {
                            scene.refresh();
                        } );
                    } );
                } )
            ;
        },
        clearControl: async function () {
            $controller.destroy( true );
            $controller = new ScrollMagic.Controller( {
                refreshInterval: 0
            } );
            for ( let s of $scene ) {
                s.destroy( true );
                s = null;
            }
            $scene = [];
        },
        nextProject: function () {
            const _next = $( '.next-project' ),
                _nextProject = _next.find( '.case img' ),
                _img = _next.find( '.bg img' ),
                heroTitle = _next.find( '.title' ),
                tween = gsap.timeline();
            if ( !_next.length ) return;
            if ( _nextProject.length )
                tween.to( _nextProject, { rotation: 360 }, 0 );
            if ( _img.length )
                tween.to( _img, { scale: 1 }, 0 );
            const parallaxIt1 = dsnGrid.tweenMaxParallax( $controller ).addParrlax( {
                id: _next,
                triggerHook: 1,
                duration: "95%",
                tween: gsap.timeline( { yoyo: true } ).fromTo( _img, {
                    y: -100, scale: 1, yoyo: true, overwrite: "none",
                }, {
                    y: 0,
                    scale: 1.2
                }, 0 )
                    .fromTo( _next.find( '.project-number , .metas' ), {
                        y: -100, yoyo: true, overwrite: "none",

                    }, { y: 0 }, 0 )
            } );
            if ( parallaxIt1 )
                $scene.push( parallaxIt1 );
            if ( dsnGrid.isMobile() )
                return;
            parallaxIt1.on( 'progress', function ( $s ) {
                _img.css( {
                    filter: "blur(" + ( ( $s.progress * 10 ) ) + "px)"
                } )
            } );
            if ( heroTitle.length ) {
                tween.to(
                    heroTitle.find( "span.d-none" ),
                    {
                        yoyo: true,
                        overwrite: "none",
                        width: "100%",
                    }, 0 );
            }
            const parallaxIt = dsnGrid.tweenMaxParallax( $controller ).addParrlax( {
                id: _next,
                triggerHook: 0,
                duration: 1000,
                tween: tween,
                _fixed: true
            } );
            parallaxIt.on( 'progress', function ( $s ) {
                _next.find( '.case .number' ).text( ( ( $s.progress ) * 100 ).toFixed( 0 ) + "%" );
                _img.css( {
                    filter: "blur(" + ( 10 - ( $s.progress * 10 ) ) + "px)"
                } )
                if ( $s.progress > 0.998 ) {
                    _next.find( 'a' ).click();
                }
            } );
            if ( parallaxIt )
                $scene.push( parallaxIt );

        },
        parallaxImg: async function () {
            let _parent = this;
            $( "[data-dsn-grid=\"move-up\"]" ).each( function () {
                const tl = gsap.timeline( { yoyo: true } );
                _parent.tweenImage( $( this ), tl );
                let parallaxIt = dsnGrid.tweenMaxParallax( $controller ).addParrlax( {
                    id: this,
                    triggerHook: dsnGrid.getData( this, "triggerhook", 1 ),
                    duration: dsnGrid.getData( this, "duration", "200%" ),
                    tween: tl,
                } );
                if ( parallaxIt )
                    $scene.push( parallaxIt );
                parallaxIt = null;
            } );
        },
        tweenImage: function ( _that, _gsap ) {
            let
                img = _that.find( "img:not(.hidden) , video" );
            _that.attr( "data-dsn-grid", "moveUp" );
            if ( img.length > 0 ) {
                let
                    speed = dsnGrid.getData( img, "speed", '180' ),
                    obj = {
                        scale: 1,
                        y: 0,
                        yoyo: true,
                        ease: easeAnimate,
                        overwrite: "none",

                    };
                gsap.set( img, {
                    height: '+='+speed,
                    y:  img.hasClass( "has-opposite-direction" ) ? '+='+speed : '-='+speed ,
                }, 0 );
                if ( img.hasClass( "has-scale" ) ) {
                    obj[ 'scale' ] = 1.1;
                }
                img.css( "perspective", _that.width() > 1000 ? 1000 : _that.width() );
                _gsap.to( img, 1, obj, 0 )
            }
        },
        parallaxMulti: async function () {
            let _parent = this;
            $( "[data-dsn-animate-multi]" ).each( function () {
                dsnGrid.getData( this, 'animate-multi' );
                const tl = gsap.timeline( {
                    yoyo: true, overwrite: "none",
                } );
                $( this ).find( "[data-dsn-grid=\"move-up\"]" ).each( function () {
                    _parent.tweenImage( $( this ), tl );

                } );
                $( this ).find( "[data-dsn-grid=\"move-section\"]" ).each( function () {
                    _parent.tweenMoveSection.bind( this, tl )();
                } );
                let duration = dsnGrid.getData( this, "duration", "200%" );
                let triggerHook = dsnGrid.getData( this, "triggerhook", 1 );
                if ( duration == 0 )
                    duration = $( this ).outerHeight() * ( triggerHook + 1 );
                let parallaxIt = dsnGrid.tweenMaxParallax( $controller ).addParrlax( {
                    id: this,
                    triggerHook: triggerHook,
                    duration: duration,
                    tween: tl,
                } );
                if ( parallaxIt )
                    $scene.push( parallaxIt );
                parallaxIt = null;
            } );
        },
        animations: async function () {
            let _parent = this;
            $( "[data-dsn-animate=\"section\"]" ).each( function () {
                dsnGrid.getData( this, "animate" );
                const _target = {
                    $this: $( this ),
                    gsap: gsap.timeline( {
                        paused: true, ease: easeAnimate, overwrite: "none",
                    } )
                }
                _parent.animateFade( _target, $( this ).find( ".dsn-up" ) )
                    .then( () => {
                        _parent.animateText( _target, $( this ).find( ".dsn-text" ) );
                    } )
                    .then( () => {
                        if ( $( this ).find( '.line' ).length )
                            _parent.animateLine( _target );
                    } )
                    .then( () => {
                        _parent.animateSkills( _target, $( this ).find( '.skills-item .fill' ) );
                    } )
                    .then( () => {
                        _parent.animateNumbers( _target, $( this ).find( ".has-animate-number" ) );
                    } )
                    .then( () => {
                        _target.gsap._totalDuration = 1;
                        const parallax = dsnGrid.tweenMaxParallax().addParrlax( {
                            id: this,
                            reverse: false,
                            triggerHook: 0.5,
                            duration: 0,
                            tween: _target.gsap,
                        } );
                        if ( _target.$this.find( '.circular-item .circle' ).length ) {
                            _target.$this.find( '.circular-item .circle' ).circleProgress( {
                                size: 160,
                                lineCap: "round",
                                startAngle: -Math.PI,
                                fill: {
                                    gradient: [ "#11468b", "#14bfb5" ]
                                },
                            } );
                            parallax.on( 'start', function () {
                                _target.$this.find( '.circular-item .circle' ).circleProgress( {} ).on( 'circle-animation-progress', function ( event, progress ) {
                                    $( this ).find( 'h4' ).html( Math.round( ( event.target.dataset.value * progress ) * 100 ) + "%" );
                                } );
                            } );
                        }
                    } )
                ;
            } );
        },
        animateFade: async function ( _target, _a, delay = 0 ) {
            if ( _a.length ) {
                _target.gsap.staggerFromTo( _a, 0.8, { y: 20, opacity: 0 }, {
                    y: 0,
                    opacity: 1,
                    delay: delay,
                    overwrite: "none",
                    ease: Back.easeOut.config( 1.7 ),
                }, 0.2, 0 );
            }
        },
        animateText: function ( _target, text ) {
            if ( text.length ) {
                text.each( function () {
                    dsnGrid.convertTextLine( this, 'words' );
                    $( this ).addClass( "overflow-hidden" );
                    _target.gsap.staggerFrom( $( this ).find( ".dsn-word-wrapper" ), 0.6,
                        {
                            willChange: "transform",
                            transformOrigin: 'left',
                            opacity: 0,
                            scaleX: 2,
                            ease: Back.easeOut.config( 2 )
                        }, 0.1, 0 );
                } );
            }
        },
        animateLine: function ( _target, _line ) {
            _target.gsap.addLabel( 'line', 0 );
            if ( _target.$this.find( '.line.line-top' ).length )
                _target.gsap.from( _target.$this.find( '.line.line-top' ).toArray(), 1, {
                    scaleX: 0,
                    transformOrigin: 'right',
                }, 'line' );
            if ( _target.$this.find( '.line.line-left' ).length )
                _target.gsap.from( _target.$this.find( '.line.line-left' ).toArray(), 1, {
                    scaleY: 0,
                    transformOrigin: 'top'
                }, 'line+=1' );
            if ( _target.$this.find( '.line.line-bottom' ).length )
                _target.gsap.from( _target.$this.find( '.line.line-bottom' ).toArray(), 1, {
                    scaleX: 0,
                    transformOrigin: 'left'
                }, 'line+=2' );
            if ( _target.$this.find( '.line.line-right' ).length )
                _target.gsap.from( _target.$this.find( '.line.line-right' ).toArray(), 1, {
                    scaleY: 0,
                    transformOrigin: 'bottom'
                }, 'line+=3' );
        },

        animateSkills: function ( _target, skills ) {
            skills.each( function ( $index ) {
                let c = $( this );
                _target.gsap.to( c, 1, {
                    width: c.data( "width" ),
                    onUpdate: function () {
                        c.find( ".number" ).text( ( c.width() / c.parent().width() * 100 ).toFixed( 0 ) + "%" );
                    },
                    onComplete: function () {
                        c = null;
                    },
                }, $index * 0.2 );

            } );
        },
        animateNumbers: function ( _target, _numbers ) {
            _target.gsap.addLabel( 'number', 0 );
            _numbers.each( function ( $index ) {
                let c = $( this );
                let numbers = { value: 0 };
                _target.gsap.to( numbers, 4, {
                    value: c.text(), ease: Back.easeOut.config( 1.2 ),

                    onUpdate: function () {
                        c.text( dsnGrid.numberText( numbers.value.toFixed( 0 ) ) );
                    },
                    onComplete: function () {
                        c = numbers = null;
                    },
                }, 'number+=' + $index * 0.2 );
            } );
        },
        headerPages: function () {
            $( '.dsn-header-animation' ).each( function () {
                let $this = $( this ),
                    heroImg = $this.find( ".dsn-hero-parallax-img" ),
                    heroTitle = $this.find( ".dsn-hero-parallax-title" );
                const parallax = gsap.timeline();
                if ( heroImg.length ) {
                    parallax.to( heroImg, {
                        y: "30%", yoyo: true, ease: easeAnimate, overwrite: "none",
                    }, 0 );
                }
                if ( heroTitle.length ) {
                    parallax.to( heroTitle, {
                        y: "-15%",
                        autoAlpha: 0,
                        yoyo: true,
                        ease: easeAnimate,
                        overwrite: "none",
                    }, 0 );
                }
                let parallaxPage = dsnGrid.tweenMaxParallax( $controller ).addParrlax( {
                    id: this,
                    triggerHook: 0,
                    duration: '100%',
                    tween: parallax
                } );
                if ( parallaxPage ) {
                    $scene.push( parallaxPage );
                }
                parallaxPage = heroImg = $this = undefined;
            } );

        },
        moveSection: function () {
            let _parent = this;
            $( "[data-dsn-grid=\"move-section\"]" ).each( function () {
                let _that = $( this );
                const tl = gsap.timeline( { yoyo: true } );
                _parent.tweenMoveSection.bind( this, tl )();
                const parallaxIt = dsnGrid.tweenMaxParallax( $controller ).addParrlax( {
                    id: this,
                    triggerHook: dsnGrid.getData( _that, "triggerhook", 1 ),
                    duration: dsnGrid.getData( _that, "duration", "150%" ),
                    tween: tl,
                } );
                $scene.push( parallaxIt );
                _that = null;
            } );
        },
        tweenMoveSection: function ( _gsap ) {
            let _that = $( this );
            dsnGrid.getData( this, "grid" );
            _that.addClass( "dsn-move-section" );
            if ( dsnGrid.getData( this, "responsive" ) === "tablet" && dsnGrid.isMobile() ) return;
            _gsap.to( _that, 1, {
                y: dsnGrid.getData( _that, "move", -150 ),
                autoAlpha: dsnGrid.getData( _that, "opacity", _that.css( "opacity" ) ),
                ease: easeAnimate,
                yoyo: true,
                overwrite: "none",
            }, 0 )
        },
        parallaxImgHover: function () {
            if ( dsnGrid.isMobile() )
                return;
            $( "[data-dsn=\"parallax\"]" ).each( function () {
                let _that = $( this );
                _that.removeAttr( "data-dsn" );
                dsnGrid.parallaxMoveElement( _that, dsnGrid.getData( _that, "move", 20 ), dsnGrid.getData( _that, "speed", 0.5 ), _that.find( ".dsn-parallax-rev" ).get( 0 ), _that.hasClass( "image-zoom" ) );
                _that = null;

            } );
        },
        dsnScrollTop: function () {
            const wrap = $( ".wrapper" );
            if ( !wrap.length || !$( ".scroll-to-top" ).length ) {
                return;
            }
            gsap.to( ".scroll-to-top", 1, { right: -100, autoAlpha: 0 } );
            const parallaxIt = dsnGrid.tweenMaxParallax( $controller ).addParrlax( {
                id: wrap,
                triggerHook: wrap.offset().top > 50 ? 0.5 : 0,
                offset: wrap.offset().top > 50 ? 0 : 100,
                duration: ( wrap.height() - ( $wind.height() * 0.5 ) ) + ( $( ".next-project" ).outerHeight() || 0 ) - ( wrap.offset().top > 50 ? 0 : 300 ),
                tween: gsap.to( ".scroll-to-top > img", 0.3, { rotation: wrap.height() / 2 } ),
            } );
            parallaxIt.on( "progress", function ( $s ) {
                $( ".scroll-to-top .box-numper span" ).text( ( ( $s.progress ) * 100 ).toFixed( 0 ) + "%" );
            } );
            parallaxIt.on( "enter", function () {
                gsap.to( ".scroll-to-top", 1, { right: 10, autoAlpha: 1 } );
            } );
            parallaxIt.on( "leave", function () {
                gsap.to( ".scroll-to-top", 1, { right: -100, autoAlpha: 0 } );
            } );
            if ( parallaxIt )
                $scene.push( parallaxIt );
        },
        translateSection: function () {
            $( '.section-image.section-move-image .transform-move-section' ).each( function () {
                const tl = gsap.timeline();
                let width = 0;  
                $( this ).find( '.swiper-slide' ).each( function () {
                    width += $( this ).outerWidth();
                } );
                $( this ).append( $( this ).find( '.swiper-slide' ).clone() );
                $( this ).append( $( this ).find( '.swiper-slide' ).clone() );
                width -= $( this ).width();
                if ( $( this ).hasClass( 'move-left' ) )
                    tl.to( this, { x: width * -1 } );
                else
                    tl.from( this, { x: width * -1 } );
                let parallaxIt = dsnGrid.tweenMaxParallax( $controller ).addParrlax( {
                    id: this,
                    triggerHook: dsnGrid.getData( this, "triggerhook", 1 ),
                    duration: dsnGrid.getData( this, "duration", "200%" ),
                    tween: tl,
                } );   
                if ( parallaxIt )
                    $scene.push( parallaxIt );
            } );
        }
    };
}
function navMenu() {
    const siteHeader = $( ".site-header" );
    return {
        init: function () {
            if ( !siteHeader.length ) return;
            let $parent = this;
            this.cutterText();
            $parent.hamburgerOpen();
        },
        cutterText: function () {
            let text_menu = siteHeader.find( ".menu-icon .text-menu" );
            if ( text_menu.length <= 0 ) return;
            let text_button = text_menu.find( ".text-button" );
            let text_open = text_menu.find( ".text-open" );
            let text_close = text_menu.find( ".text-close" );  
            dsnGrid.convertTextLine( text_button );
            dsnGrid.convertTextLine( text_open );
            dsnGrid.convertTextLine( text_close );
            text_close = null;
            text_open = null;
            text_button = null;
            text_menu = null;
        },
        hamburgerOpen: function () {
            const mainIcon = siteHeader.find( ".menu-icon" );
            const mainNav = siteHeader.find( ".main-navigation" );
            let tl = gsap.timeline( {
                paused: true, onReverseComplete: function () {
                    setTimeout( function () {
                        mainIcon.find( ".icon-top , .icon-bottom" ).css( "transform", "" ).css( "display", "" );
                    }, 50 ); 
                    console.log( 'onReverseComplete : tl' );
                },
            } );
            var menuClick = gsap.timeline( {
                onReverseComplete: function () {
                    menuClick = gsap.timeline();
                    console.log( 'onReverseComplete : menuClick' );
                }
            } );
            let Ease = Power3.easeOut;
            tl.set( mainIcon.find( ".icon-center" ), { display: "none" } );
            tl.to( mainIcon.find( ".icon-top" ), 0.5, { width: 23, rotation: 45, top: 0, ease: Ease } );
            tl.to( mainIcon.find( ".icon-bottom" ), 0.5, {
                width: 23,
                rotation: -45,
                top: 0,
                ease: Ease,
            }, 0 );
            tl.call( function () {
                mainIcon.toggleClass( 'nav-active' );
            }, 0 );      
            tl.to( mainNav, 0.5, { y: "0%", autoAlpha: 1, ease: Ease }, 0 );
            tl.fromTo( mainNav, 0.5, { y: "-100%", autoAlpha: 0 }, {
                y: "0%",
                autoAlpha: 1,
                ease: Expo.easeInOut,
            }, 0 );       
            tl.staggerTo( mainNav.find( "ul.extend-container > li > a .dsn-title-menu" ), 0.5, {
                autoAlpha: 1,
                y: 0,
                ease: Back.easeOut.config( 1.7 ),
            }, 0.05 );       
            tl.set( mainNav.find( "ul.extend-container > li > a .dsn-meta-menu" ), {
                autoAlpha: 1,
                ease: Ease,
            } );
            tl.to( mainNav.find( ".container-content" ), 1, { autoAlpha: 1 }, "-=1" );
            tl.reverse();        
            mainNav.find( "ul.extend-container > li.dsn-drop-down" ).on( "click", function ( e ) {
                e.stopPropagation();       
                if ( menuClick._tDur > 0 ) return;
                menuClick = gsap.timeline( {
                    onReverseComplete: function () {
                        menuClick = gsap.timeline();
                    },
                } );
                menuClick.set( $( this ).find( "ul" ), { display: "flex" } );
                menuClick.to( mainNav.find( "ul.extend-container > li > a " ).find( ".dsn-title-menu , .dsn-meta-menu" ), 0.5,
                    { y: -30, autoAlpha: 0, ease: Back.easeIn.config( 1.7 ) } );
                menuClick.set( ".site-header .extend-container .main-navigation ul.extend-container li", { overflow: "hidden" } );
                menuClick.staggerFromTo( $( this ).find( "ul li" ), 0.5, { x: 50, autoAlpha: 0 }, {
                    x: 0,
                    autoAlpha: 1,
                    ease: Back.easeOut.config( 1.7 ),
                }, 0.1 );      
            } );        
            mainIcon.off( "click" );
            mainIcon.on( "click", function () {
                console.log( 'mainIcon:click' );    
                if ( !tl.isActive() ) {
                    menuClick.reverse( -1 );
                    tl.reversed( !tl.reversed() );
                    menuClick = gsap.timeline();
                }
            } );
            let backMenu = $( ".dsn-back-menu" );
            backMenu.off( "click" );
            backMenu.on( "click", function ( e ) {
                console.log( 'backMenu:click' );
                e.stopPropagation();
                menuClick.reverse();

            } );
        },
    };
}

  function slider() {
    const dsn_slider = $( ".main-slider" );
    let tl = gsap.timeline();
    return {
        run: async function () {
            let $slidObject = this;
            dsn_slider.each( function () {
                let $this = $( this ),
                    horizontal = $this.hasClass( "has-horizontal" ),
                    inner = $this.find( ".slide-inner" );
                if ( inner.hasClass( 'dsn-webgl' ) ) {
                    $slidObject.initWebgel( $( this ) ).then( ( $obj ) => {
                        dsn_slider.find( ".control-nav .slider-total-index" ).html( dsnGrid.numberText( $obj.imgs.length ) );
                        dsnGrid.WebGLDistortionHoverEffects( $obj, {
                            parent: inner,
                            vertical: !horizontal,
                            nextEl: dsn_slider.find( ".next-container" ),
                            prevEl: dsn_slider.find( ".prev-container" ),
                            onComplete: function () {

                            },
                            onStart: function ( current, oldIndex ) {
                                $slidObject.slideChangeWeb( dsn_slider, horizontal ? "x" : "y", current, oldIndex, this.mat.uniforms.effectFactor.value < 0 );
                            },
                        } );
                    } );
                } else if ( inner.find( ".slide-item" ).length ) {
                    $slidObject.initSlider( $this ).then( function ( $d ) {
                        let swiper = $slidObject.swiperObject( $this, !horizontal );
                        $slidObject.slideChange( swiper, $this, horizontal ? "x" : "y" );
                        dsnGrid.addSwiper( swiper );
                        console.log( swiper.getTranslate() );
                        $this.find( '.next-container' ).on( "click", function () {
                            if ( tl.isActive() ) return;
                            swiper.slideNext();
                        } );
                        $this.find( '.prev-container' ).on( "click", function () {
                            if ( tl.isActive() ) return;
                            swiper.slidePrev();
                        } );
                    } );  
                }
            } );
        },
        initSlider: async function ( dsn_slider ) {
            dsn_slider.find( ".slide-item" ).each( function ( $index ) {
                let $this = $( this );
                $this.attr( "data-dsn-id", $index );
                let slide_content = $( this ).find( ".slide-content" );
                slide_content.attr( "data-dsn-id", $index );
                if ( $index === 0 ) slide_content.addClass( "dsn-active dsn-active-cat" );
                dsn_slider.find( ".dsn-slider-content > .dsn-container" ).append( slide_content );

                let title = slide_content.find( ".title" );
                if ( title.find( "a" ).length )
                    title = title.find( "a" );

                dsnGrid.convertTextLine( title );
                $this = slide_content = title = null;
            } );
        },
        swiperObject: function ( dsn_slider, $isVertical = true ) {
            return new Swiper( dsn_slider.find( ".slide-inner" ).get( 0 ), {
                speed: 1000,
                grabCursor: true,
                allowTouchMove: true,
                direction: $isVertical ? "vertical" : "horizontal",
                slidesPerView: 1,
                parallax: true,
                loop: true,
                loopAdditionalSlides: 10,
                watchSlidesProgress: true,
                watchSlidesVisibility: true,
                pagination: {
                    el: dsn_slider.find( ".slider-current-index" ).get( 0 ),
                    type: "custom",
                    clickable: true,
                    renderCustom: function ( swiper, current, total ) {
                        dsn_slider.find( ".slider-total-index" ).html( dsnGrid.numberText( total ) );
                        return dsnGrid.numberText( current );
                    },
                },
                on: {
                    init: function () {
                        this.autoplay.stop();
                        let swiper = this;
                        dsn_slider.find( "[data-dsn=\"video\"] video" ).each( function () {
                            this.pause();
                        } );
                        this.touchEventsData.formElements = "*";
                        dsn_slider.find( '[data-swiper-parallax]' ).each( function () {
                            let num = $( this ).attr( 'data-swiper-parallax' ).replace( '%', '' );
                            $( this ).attr( {
                                'data-swiper-parallax': ( num / 100 ) * ( $isVertical ? swiper.height : swiper.width )
                            } )
                        } );
                    },
                    touchStart: function () {
                        $( this.slides ).css( "transition", "" );
                        if ( !dsnGrid.isMobile() && $body.hasClass( 'dsn-cursor-effect' ) ) {
                            if ( !$( this.slides ).parents( '.main-slider' ).hasClass( 'has-horizontal' ) ) {
                                $( '.cursor' ).addClass( 'cursor-scale-half cursor-up-down cursor-drag cursor-next cursor-prev' );
                            } else {
                                $( '.cursor' ).addClass( 'cursor-scale-half cursor-drag cursor-next cursor-prev' );
                            }
                        }
                    },
                    touchEnd: function () {
                        if ( !dsnGrid.isMobile() && $body.hasClass( 'dsn-cursor-effect' ) ) {
                            if ( !$( this.slides ).hasClass( 'has-horizontal' ) ) {
                                $( '.cursor' ).removeClass( 'cursor-scale-half cursor-up-down cursor-drag cursor-next cursor-prev' );
                            } else {
                                $( '.cursor' ).removeClass( 'cursor-scale-half cursor-drag cursor-next cursor-prev' );
                            }
                        }
                    }
                },
            } );
        },

        slideChange: function ( swiper, dsn_slider, $dir ) {
            let $this = this;
            swiper.on( "slideChange", start );
            async function start() {
                let contentOld = dsn_slider.find( ".dsn-slider-content .dsn-active" );
                let numOld = contentOld.data( "dsn-id" );
                let slider = $( swiper.slides[ swiper.activeIndex ] );
                let id = slider.data( "dsn-id" );
                if ( numOld === id ) return;
                dsn_slider.find( "[data-dsn=\"video\"] video" ).each( function () {
                    this.pause();
                } );
                let v = $( this.slides[ this.activeIndex ] ).find( "[data-dsn=\"video\"] video" );
                if ( v.length ) v.get( 0 ).play();
                let content_letterOld = contentOld.find( ".dsn-chars-wrapper" );
                contentOld.removeClass( "dsn-active-cat" );
                let contentNew = dsn_slider.find( ".dsn-slider-content [data-dsn-id=\"" + id + "\"]" );
                let content_letterNew = contentNew.find( ".dsn-chars-wrapper" );
                let $isRight = numOld > id;
                tl.progress( 1 );
                tl = new gsap.timeline();
                tl.staggerFromTo(
                    $isRight ? content_letterOld.toArray().reverse() : content_letterOld,
                    0.3,
                    $this.showText( $dir ).title,
                    $this.hideText( $isRight, $dir ).title,
                    0.05,
                    0,
                    function () {
                        dsn_slider.find( ".dsn-slider-content .slide-content" ).removeClass( "dsn-active" ).removeClass( "dsn-active-cat" );
                        contentNew.addClass( "dsn-active" );
                        contentNew.addClass( "dsn-active-cat" );
                    },
                );
                tl.staggerFromTo(
                    $isRight ? content_letterNew.toArray().reverse() : content_letterNew,
                    0.8,
                    $this.hideText( !$isRight, $dir ).title,
                    $this.showText( $dir ).title,
                    0.05,
                    "-=.1",
                );
                contentOld = numOld = slider = id = v = content_letterOld = content_letterNew = $isRight = null;
            }
        },
        slideChangeWeb: function ( dsn_slider, $dir, current, oldIndex, $isRight ) {
            let $this = this;
            dsn_slider.find( ".control-nav .slider-current-index" ).html( dsnGrid.numberText( current + 1 ) );
            let contentOld = dsn_slider.find( ".dsn-slider-content .dsn-active" );
            let content_letterOld = contentOld.find( ".dsn-chars-wrapper" );
            let contentNew = dsn_slider.find( ".dsn-slider-content [data-dsn-id=\"" + current + "\"]" );
            let content_letterNew = contentNew.find( ".dsn-chars-wrapper" );
            dsn_slider.find( ".slide-inner" ).attr( "data-overlay", contentNew.data( "overlay" ) );
            tl.progress( 1 );
            tl = new gsap.timeline();
            tl.staggerFromTo(
                $isRight ? content_letterOld.toArray().reverse() : content_letterOld,
                0.3,
                $this.showText( $dir ).title,
                $this.hideText( $isRight, $dir ).title,
                0.05,
                0,
                function () {
                    dsn_slider.find( ".dsn-slider-content .slide-content" ).removeClass( "dsn-active" ).removeClass( "dsn-active-cat" );
                    contentNew.addClass( "dsn-active" );
                    contentNew.addClass( "dsn-active-cat" );
                },
            );
            tl.staggerFromTo(
                $isRight ? content_letterNew.toArray().reverse() : content_letterNew,
                0.8,
                $this.hideText( !$isRight, $dir ).title,
                $this.showText( $dir ).title,
                0.05,
                "-=.1",
            );
        },
        showText: function ( $dir ) {
            let $obj = {
                title: {
                    autoAlpha: 1,
                    scale: 1,
                    ease: "back.out(4)",
                    yoyo: true,
                },
            };
            $obj.title[ $dir ] = "0%";
            return $obj;
        },
        hideText: function ( $isRigth, $dir ) {
            let $obj = {
                title: {
                    autoAlpha: 0,
                    ease: "back.in(4)",
                    yoyo: true,
                },
            };
            $obj.title[ $dir ] = ( $isRigth ) ? "40%" : "-40%";
            return $obj;
        },
        initWebgel: async function ( dsn_slider ) {
            let imgs = [],
                overlady = [];
            dsn_slider.find( ".dsn-slider-content .slide-content" ).each( function ( $index ) {
                let slide_content = $( this );
                imgs[ $index ] = slide_content.data( "webgel-src" );
                overlady[ $index ] = slide_content.data( "overlay" );
                slide_content.attr( "data-dsn-id", $index );
                if ( $index === 0 ) slide_content.addClass( "dsn-active dsn-active-cat" );
                let title = slide_content.find( ".title a" );
                dsnGrid.convertTextLine( title );
                slide_content = title = null;
            } );
            dsn_slider.find( ".slide-inner" ).attr( "data-overlay", overlady[ 0 ] );
            return {
                imgs: imgs,
                overlay: overlady,
                displacement: dsnGrid.getData( dsn_slider.find( ".slide-inner" ), "displacement", "/static/2_images/8.jpg" ),
                intensity: dsnGrid.getData( dsn_slider.find( ".slide-inner" ), "intensity", -2 ),
                speedIn: dsnGrid.getData( dsn_slider.find( ".slide-inner" ), "speedIn", 1.2 ),
                speedOut: dsnGrid.getData( dsn_slider.find( ".slide-inner" ), "speedOut", 1.2 ),
                easing: dsnGrid.getData( dsn_slider.find( ".slide-inner" ), "easing", "Expo.easeInOut" ),
            };
        },
    };
}

  function loadData( $type ) {
    setTimeout( function () {
        $( "[data-dsn-" + $type + "]" ).each( function () {
            $( this ).attr( $type, dsnGrid.getData( this, $type, "" ) );
        } );
    }, 100 );
}
function Isotope() {
    $( ".dsn-isotope" ).each( ( $key, $item ) => {
        $( $item ).masonry( {
            itemSelector: dsnGrid.getData( $item, 'item', '.grid-item' ),
            horizontalOrder: dsnGrid.getData( $item, 'horizontalOrder', true ),
            fitWidth: dsnGrid.getData( $item, 'fitWidth', false ),
        } );
    } )
    $( '.dsn-filter' ).each( function () {
        const $button = $( this ).find( '.filtering-t ' ),
            $item = $( this ).find( '.dsn-item-filter' );
        if ( !$button.length || !$item.length ) return;
        $item.isotope();
        $button.find( "button" ).off( "click" );
        $button.find( "button" ).on( "click", function () {
            $( this ).addClass( 'active' ).siblings().removeClass( "active" );
            $item.isotope( {
                filter: $( this ).attr( "data-filter" ),
            } );
        } );
    } );
}

  function projectSlider() {
    return {
        swiper: function ( $id, $obj ) {
            $id = dsnGrid.convertToJQuery( $id );
            $obj = $.extend( true, {
                slidesPerView: 1,
                centeredSlides: true,
                spaceBetween: 0,
                grabCursor: true,
                speed: 1000,
                parallax: true,
                loop: true,
                slideToClickedSlide: true,
                pagination: {
                    el: $id.find( ".swiper-pagination" ).get( 0 ),
                    clickable: true,
                },
                navigation: {
                    nextEl: $id.find( ".swiper-next" ).get( 0 ),
                    prevEl: $id.find( ".swiper-prev" ).get( 0 ),
                },
            }, $obj );
            let $s = new Swiper( $id.find( ".swiper-container" ).get( 0 ), $obj );
            dsnGrid.addSwiper( $s );
        },
        run: function () {
            let $this = this;
            $( ".dsn-swiper" ).each( function () {
                let option = dsnGrid.getData( this, "option", {} );
                let syn = $( this ).parent().find( dsnGrid.getData( this, "controller" ) );
                if ( syn.length ) {
                    option[ 'thumbs' ] = {
                        swiper: {
                            el: syn.find( '.swiper-container' ).get( 0 ),
                            allowTouchMove: false,
                            slidesPerView: 1,
                            speed: option.speed || 1000,
                            parallax: true,
                            autoHeight: true
                        }
                    };      
                }
                option[ "breakpoints" ] = {
                    768: {
                        slidesPerView: option.slidesPerView >= 1 ? ( option.slidesPerView > 1.5 ? 2 : 1.30 ) : 1,
                        spaceBetween: option.slidesPerView > 1 ? ( option.spaceBetween > 21 ? 20 : option.spaceBetween ) : 0,
                    },
                    992: {
                        slidesPerView: option.slidesPerView,
                        spaceBetween: option.spaceBetween || 0,
                    },
                    575: {
                        slidesPerView: 1,
                        spaceBetween: 0,           
                    },
                };
                if ( syn.length ) {
                    option[ 'thumbs' ] = {
                        swiper: {
                            el: syn.find( '.swiper-container' ).get( 0 ),
                            allowTouchMove: false,
                            slidesPerView: 1,
                            speed: option.speed || 1000,
                            parallax: true,
                            autoHeight: true
                        }
                    };  
                    option.breakpoints[ '768' ] = {
                        slidesPerView: 1,
                        spaceBetween: 0,
                    }
                }
                option[ 'slidesPerView' ] = 1;
                option[ 'spaceBetween' ] = 0;
                $this.swiper( this, option );
            } );
        },
    };
}

function accordion() {    
    $( ".dsn-accordion" ).each( function () {
        let $this = $( this ),
            acc_q = $this.find( ".accordion__question" );
        acc_q.on( "click", function () {
            let content = $( this ).next();
            $this.find( ".accordion__answer" ).not( content ).slideUp( 400 );
            acc_q.not( this ).removeClass( "expanded" );
            $( this ).toggleClass( "expanded" );
            content.slideToggle( 400 );
            content = null;
        } );
    } );  
}

  async function mouseCirMove( $off ) {
    const $elemnet = $( ".cursor" );
    if ( dsnGrid.isMobile() || !$body.hasClass( "dsn-cursor-effect" ) ) {
        if ( $elemnet.length ) {
            $elemnet.css( "display", "none" );
            $body.removeClass( "dsn-cursor-effect" );
        }
        return;
    } 
    if ( $off === true ) {
        $elemnet.attr( "class", "cursor" );
        cursorEffect();
        return;
    }
    dsnGrid.mouseMove( $elemnet, {
        speed: 0.5
    } );
    cursorEffect(); 
    function cursorEffect() {
        dsnGrid.elementHover( $elemnet, "a:not(> img):not(.vid) , .dsn-button-sidebar,  button , .mfp-container", "cursor-scale-full" );
        dsnGrid.elementHover( $elemnet, ".c-hidden , .social-side a", "no-scale" );
        dsnGrid.elementHover( $elemnet, ".has-popup a , .work-item-box a:not(.effect-ajax)", "cursor-scale-half cursor-open" );
        dsnGrid.elementHover( $elemnet, "[data-cursor=\"close\"]", "cursor-scale-full cursor-close" );
        dsnGrid.elementHover( $elemnet, "a.link-pop ", "cursor-scale-full cursor-view" );
    }
}
async function linkRightPaginate() {
    const $cont = $( '.dsn-paginate-right-page' );
    if ( !$cont.length ) return;
    $cont.empty();
    $( '[data-dsn-title]' ).each( function () {
        const $title = dsnGrid.getData( this, 'title' );
        const _target = $( this ).offset().top;
        const _element = $( '<div class="dsn-link-paginate text-transform-upper"></div>' );
        _element.html( $title );
        $cont.append( _element );
        _element.on( 'click', function () {   
            dsnGrid.scrollTop( _target, 1, -150 );
        } );     
    } );   
}

function contactValidator() {
    const contact_form = $( "#contact-form" );
    if ( contact_form < 1 ) {
        return;
    }
    contact_form.off( "submit" );
    contact_form.on( "submit", function ( e ) {
        if ( !e.isDefaultPrevented() ) {
            contact_form.validator();
            $.ajax( {
                type: "POST",
                url: "contact.php",
                data: $( this ).serialize(),
                success: function ( data ) {
                    var messageAlert = "alert-" + data.type;
                    var messageText = data.message;
                    var alertBox = "<div class=\"alert " + messageAlert + " alert-dismissable\"><button type=\"button\" class=\"close\" data-dismiss=\"alert\" aria-hidden=\"true\">&times;</button>" + messageText + "</div>";
                    if ( messageAlert && messageText ) {
                        contact_form.find( ".messages" ).html( alertBox );
                        contact_form[ 0 ].reset();
                    }
                    setTimeout( function () {
                        contact_form.find( ".messages" ).html( "" );
                    }, 3000 );
                },
                error: function ( error ) {
                    console.log( error );
                },
            } );
            return false;
        }
    } );
}
function effctStickyNavBar() {
    $wind.off( "scroll" );
    const wrapper = $( '.wrapper' );
    let bodyScroll = 0;
    var $ofContent = wrapper.offset();
    var header = wrapper.find( '> *:first-child' );
    var scrDown = 0;
    if ( header.length ) {
        $ofContent = header.get( 0 ).nodeName === 'HEADER' ? header.outerHeight() : ( $ofContent !== undefined ) ? $ofContent.top : 70;
    }
    $effectScroll.getListener( function ( e ) {
        if ( e.type === "scroll" ) {
            bodyScroll = $wind.scrollTop();
        } else {
            bodyScroll = e.offset.y;
        }
        if ( $ofContent > 170 ) {
            $ofContent = $ofContent - 100;
        }
        if ( bodyScroll > $ofContent ) {
            if ( scrDown < bodyScroll ) {
                $body.addClass( 'nav-bg' ).addClass( 'hide-nav' )
            } else {
                $body.removeClass( 'hide-nav' )
            }
        } else {
            $body.removeClass( 'nav-bg' ).removeClass( 'hide-nav' );
        }
        scrDown = bodyScroll;
    } );
}
function dropHash() {
    $( "a[href=\"#\"]" ).on( "click", function ( e ) {
        e.preventDefault();
    } );
    $( "[href*=\"#\"]:not([href=\"#\"])" ).on( "click", function ( e ) {
        e.preventDefault();
        let href = $( $( this ).attr( "href" ) );
        if ( !href.length ) {
            href = null;
            return false;
        }
        dsnGrid.scrollTop( href.get( 0 ).offsetTop, 1, -100 );
        href = null;
    } );
    if ( window.location.hash.length ) {
        $wind.scrollTop( 0 );
        dsnGrid.scrollTop( window.location.hash, 1, -100 );
    }


}
function changeStyle() {
    const options = $( '.box-options' );
    options.find( '.title-mode' ).each( function () {
        dsnGrid.convertTextLine( this );
    } );
    options.find( '.day-night' ).on( 'click', function () {
        const _dark = $( '.v-dark' );
        const _light = $( '.v-light' );
        $body.toggleClass( 'v-dark' );
        _dark.removeClass( 'v-dark' ).addClass( 'v-light' );
        _light.addClass( 'v-dark' ).removeClass( 'v-light' );
    } );
    options.find( '.mode-layout' ).on( 'click', function () {
        $body.toggleClass( 'dsn-line-style' );
        for ( let s of $build.swiper ) {
            s.update();
        }
        Isotope();
    } );
}
function dsnAjax() {
    const main_root = "main.main-root",
        text_e_img = "[data-dsn-ajax=\"img\"]",
        text_e_title = "[data-dsn-ajax=\"title\"]";
    var img_move, title_move,
        tl = gsap.timeline();
    return {
        ajaxLoad: function () {
            if ( !$body.hasClass( "dsn-ajax" ) ) return;
            let $parent = this;
            this.ajaxClick.off( "click" );
            this.ajaxClick.on( "click", function ( e ) {
                e.preventDefault();
                let _that = $( this ),
                    url = _that.attr( "href" ),
                    _type = _that.data( "dsn-ajax" );
                if ( url.indexOf( "#" ) >= 0 || url === undefined ) {
                    _that = url = _type = null;
                    return;
                }
                if ( $parent.effectAjax() ) return;
                $parent.effectAjax( true );
                $.ajax( {
                    url: url,
                    dataType: "html",
                    beforeSend: $parent.animateAjaxStart.bind( $parent, _type, _that ),
                    success: function ( data ) {
                        try {
                            history.pushState( null, "", url );
                            tl.call( $parent.animateAjaxEnd.bind( $parent, data ), null, null, "+=0.2" );
                        } catch ( e ) {
                            window.location = url;
                        }
                    }, error: function ( error ) {
                        window.location = url;
                    },
                } );
            } );
        },
        mainRoot: $( main_root ),
        ajaxClick: $( "a.effect-ajax " ),
        effectAjax: function ( $add ) {
            if ( $add )
                $body.addClass( "dsn-ajax-effect" );
            else if ( $add === false )
                $body.removeClass( "dsn-ajax-effect" );
            else
                return $body.hasClass( "dsn-ajax-effect" );
        },
        animateAjaxStart: function ( _type, _that ) {
            tl.clear();
            tl.addLabel( 'beforeSend' );
            if ( dsnGrid.isMobile() && _type === 'next' )
                _type = undefined;


            switch ( _type ) {
                case 'slider' :
                    this.ajaxSlider( _that );
                    break;
                case 'next' :
                    this.ajaxNextProject( _that );
                    break;
                case 'work' :
                    this.ajaxWork( _that );
                    break;
                case 'work-hover' :
                    this.ajaxWorkHover( _that );
                    break;
                default :
                    this.ajaxNormal();
            }
            if ( _type !== 'next' ) {
                $effectScroll.locked();
                tl.call( function () {
                    dsnGrid.scrollTop( 0, 0.01 );
                } );
            }
        },
        ajaxNormal: function () {
            let elemnt_ajax = $( "<div class=\"dsn-ajax-loader dsn-ajax-normal\"></div>" );
            $body.append( elemnt_ajax );
            tl.to( elemnt_ajax, 1, { autoAlpha: 1, ease: Expo.easeOut }, 0 );
            elemnt_ajax = null;
        },
        ajaxSlider: function ( $e ) {
            let
                active = $e.parents( ".slide-content" ),
                id = active.data( "dsn-id" ),
                img = active.parents( '.main-slider' ).find( ".slide-item[data-dsn-id=\"" + id + "\"] .cover-bg" ).first(),
                title = active.find( ".title" );
            let bg_con = active.parents( '.main-slider' ).find( '.bg-container' );
            img.removeClass( "hidden" );
            if ( active.data( 'webgel-src' ) )
                img = $( "<div class='cover-bg'></div>" ).attr( {
                    'data-overlay': bg_con.find( '.dsn-webgl' ).data( 'overlay' ),
                    'style': 'background-image:url("' + active.data( 'webgel-src' ) + '")'
                } );
            this.dsnCreateElement( img, bg_con, title, title );
        },
        ajaxNextProject: function ( $e ) {
            let
                active = $e.parents( '.next-project' ),
                img = active.find( ".bg" ),
                title = $e;
            const effectS = window.Scrollbar.get( document.querySelector( "#dsn-scrollbar" ) );

            tl.to( effectS || $wind, 1, {
                scrollTo: { y: effectS ? $effectScroll.getScrollbar().limit.y : document.body.scrollHeight },
            } );
            tl.call( this.dsnCreateElement.bind( this, img, active, title.find( '.title' ), title, {
                before: function ( container, img_move, title_move ) {
                    title_move.removeClass( 'border-top' ).removeClass( 'border-bottom' );
                }
            } ) );
            tl.call( function () {
                $effectScroll.locked();
                tl.call( function () {
                    dsnGrid.scrollTop( 0, 0.01 );
                } );
            } )
            active = img = title = null;
        },
        ajaxWork: function ( $e ) {
            let
                active = $e.parents( ".work-item" ),
                img = active.find( ".box-img" ),
                title = active.find( ".sec-title" );
            this.dsnCreateElement( img, img, title, title );
            tl.to( img_move.find( "img" ), 0.5, { height: "100%", top: "0%", y: "0" } );
            active = img = title = null;
        },
        ajaxWorkHover: function ( $e ) {
            let
                active = $e,
                img = active.find( ".hover-reveal" ),
                title = active.find( ".work__item-text" );
            this.dsnCreateElement( img.find( '.hover-reveal__img' ), img, title, title );
            active = img = title = null;
        },
        addElement: function ( container, $e, $target ) {
            if ( $e === undefined || $e.length <= 0 ) return undefined;
            if ( $target === undefined || $target.length <= 0 ) {
                $target = $e;
            }
            $e.removeClass( "line-after" ).removeClass( "line-before" );
            let $section = $e.clone();
            let position = $target[ 0 ].getBoundingClientRect();
            if ( position === undefined ) {
                position = {
                    left: 0,
                    top: 0,
                };
            }
            $section.css( {
                position: "fix",
                display: "block",
                transform: "",
                transition: "",
                objectFit: "cover",
            } );
            $section.css( dsnGrid.getBoundingClientRect( $target[ 0 ] ) );
            $section.css( $e.dsnGridStyleObject() );
            container.append( $section );
            return $section;
        },
        dsnCreateElement: function ( $e, $target, $letter, $targetLtter, $object = {} ) {
            let container = $( "<div class=\"dsn-ajax-loader\"></div>" );
            img_move = this.addElement( container, $e, $target );
            title_move = this.addElement( container, $letter, $targetLtter );
            if ( !title_move.find( ".dsn-chars-wrapper" ).length ) dsnGrid.convertTextLine( title_move );
            if ( $object.before !== undefined )
                $object.before( container, img_move, title_move );
            $body.append( container );
            tl.to( container, 1, { autoAlpha: 1, ease: Power4.easeInOut }, '-=0.8' );
            if ( $object.after !== undefined )
                $object.after( container, img_move, title_move );
        },
        completeElement: function ( elAjax ) {
            let img = $( text_e_img );
            let title = $( text_e_title );
            if ( !img.length && !title.length ) {
                let webkitClipPath = { value: "0%" };
                tl.to( webkitClipPath, 1, {
                    value: "100%",
                    onUpdate: function () {
                        elAjax.css( "clip-path", "inset(0% 0% " + webkitClipPath.value + " 0%)" );
                    },
                    onComplete: function () {
                        webkitClipPath = null;
                    },
                    ease: Circ.easeIn,
                } );
                return;
            }
            img = img.first();
            let position = {
                top: 0,
                left: 0,
                width: "100%",
                height: "100%",
                transform: "none",
            };
            if ( title_move.length ) {
                title = title.first();
                if ( !title.find( ".dsn-chars-wrapper" ).length ) dsnGrid.convertTextLine( title );
                position = title.offset();
                if ( position === undefined ) {
                    position = {
                        top: 0,
                        left: 0,
                    };
                }
                tl.set( title_move.find( ".dsn-chars-wrapper" ), {
                    x: title_move.offset().left - position.left,
                    y: title_move.offset().top - position.top,
                }, "-=1" );

                let trans_title = title_move.find( ".dsn-chars-wrapper" ).toArray();
                if ( title_move.offset().left < position.left )
                    trans_title.reverse();
                tl.set( title_move, { top: position.top, left: position.left }, "-=0.8" );
                tl.to( title_move, 0.4, {
                    padding: "0", borderWidth: 0, yoyo: true
                } );
                tl.to( title_move, 0.8, {
                    css: title.dsnGridStyleObject(), yoyo: true
                }, "-=0.8" );
                title_move.css( "width", title.outerWidth() );
                tl.set( trans_title, { color: title_move.css( 'color' ) } );
                tl.staggerTo(
                    trans_title,
                    0.8,
                    {
                        y: "0",
                        x: "0",
                        ease: Back.easeOut.config( 1 ),
                        color: title.css( 'color' ),
                        yoyo: true
                    }, 0.02, "-=0.35" );
            }
            if ( img.length )
                position = {
                    top: img.get( 0 ).offsetTop,
                    left: img.get( 0 ).offsetLeft,
                    width: img.width(),
                    height: img.height(),
                };
            if ( img_move.length )
                tl.to( img_move, {
                    duration: 1,
                    top: position.top,
                    left: position.left,
                    width: position.width,
                    height: position.height,
                    objectFit: "cover",
                    borderRadius: 0,
                    ease: Expo.easeIn,
                }, '-=1.4' );
            let webkitClipPath = { value: "0%" };
            tl.to( webkitClipPath, 0.5, {
                value: "100%",
                onUpdate: function () {
                    elAjax.css( "clip-path", "inset(0% 0% " + webkitClipPath.value + " 0%)" );
                },
                onComplete: function () {
                    webkitClipPath = null;
                },
                ease: Circ.easeIn,
            } );
        },
        animateAjaxEnd: function ( data ) {
            tl.call( function () {
                dsnGrid.initAjax( data );
                this.mainRoot.html( $( data ).filter( main_root ).html() );
                reloadAjax( true ).catch( $err => {
                    console.error( $err );
                } );
            }.bind( this ), null, '+=1' );
            let elAjax = $( ".dsn-ajax-loader" );
            if ( elAjax.hasClass( "dsn-ajax-normal" ) )
                tl.to( elAjax, 1, { autoAlpha: 0, ease: Expo.easeIn } );
            else
                tl.call( this.completeElement.bind( this, elAjax ) );
            tl.eventCallback( "onComplete", function () {
                elAjax.remove();
                this.effectAjax( false );
            }.bind( this ) );
        },
        backAnimate: function ( url ) {
            if ( !url ) return;
            let $parent = this;
            $.ajax( {
                url: url,
                dataType: "html",
                beforeSend: $parent.animateAjaxStart.bind( $parent ),
                success: function ( data ) {
                    tl.call( $parent.animateAjaxEnd.bind( $parent, data ), null, null, "+=0.2" );
                }, error: function ( error ) {
                    window.location = url;
                },
            } );
        },
    };
}


  check_first_load();