
(function(l, r) { if (!l || l.getElementById('livereloadscript')) return; r = l.createElement('script'); r.async = 1; r.src = '//' + (self.location.host || 'localhost').split(':')[0] + ':35729/livereload.js?snipver=1'; r.id = 'livereloadscript'; l.getElementsByTagName('head')[0].appendChild(r) })(self.document);
(function () {
    'use strict';

    function noop() { }
    function add_location(element, file, line, column, char) {
        element.__svelte_meta = {
            loc: { file, line, column, char }
        };
    }
    function run(fn) {
        return fn();
    }
    function blank_object() {
        return Object.create(null);
    }
    function run_all(fns) {
        fns.forEach(run);
    }
    function is_function(thing) {
        return typeof thing === 'function';
    }
    function safe_not_equal(a, b) {
        return a != a ? b == b : a !== b || ((a && typeof a === 'object') || typeof a === 'function');
    }
    function is_empty(obj) {
        return Object.keys(obj).length === 0;
    }
    function append(target, node) {
        target.appendChild(node);
    }
    function insert(target, node, anchor) {
        target.insertBefore(node, anchor || null);
    }
    function detach(node) {
        node.parentNode.removeChild(node);
    }
    function destroy_each(iterations, detaching) {
        for (let i = 0; i < iterations.length; i += 1) {
            if (iterations[i])
                iterations[i].d(detaching);
        }
    }
    function element(name) {
        return document.createElement(name);
    }
    function text(data) {
        return document.createTextNode(data);
    }
    function space() {
        return text(' ');
    }
    function listen(node, event, handler, options) {
        node.addEventListener(event, handler, options);
        return () => node.removeEventListener(event, handler, options);
    }
    function attr(node, attribute, value) {
        if (value == null)
            node.removeAttribute(attribute);
        else if (node.getAttribute(attribute) !== value)
            node.setAttribute(attribute, value);
    }
    function to_number(value) {
        return value === '' ? null : +value;
    }
    function children(element) {
        return Array.from(element.childNodes);
    }
    function set_input_value(input, value) {
        input.value = value == null ? '' : value;
    }
    function custom_event(type, detail, { bubbles = false, cancelable = false } = {}) {
        const e = document.createEvent('CustomEvent');
        e.initCustomEvent(type, bubbles, cancelable, detail);
        return e;
    }

    let current_component;
    function set_current_component(component) {
        current_component = component;
    }
    function get_current_component() {
        if (!current_component)
            throw new Error('Function called outside component initialization');
        return current_component;
    }
    function onMount(fn) {
        get_current_component().$$.on_mount.push(fn);
    }

    const dirty_components = [];
    const binding_callbacks = [];
    const render_callbacks = [];
    const flush_callbacks = [];
    const resolved_promise = Promise.resolve();
    let update_scheduled = false;
    function schedule_update() {
        if (!update_scheduled) {
            update_scheduled = true;
            resolved_promise.then(flush);
        }
    }
    function add_render_callback(fn) {
        render_callbacks.push(fn);
    }
    // flush() calls callbacks in this order:
    // 1. All beforeUpdate callbacks, in order: parents before children
    // 2. All bind:this callbacks, in reverse order: children before parents.
    // 3. All afterUpdate callbacks, in order: parents before children. EXCEPT
    //    for afterUpdates called during the initial onMount, which are called in
    //    reverse order: children before parents.
    // Since callbacks might update component values, which could trigger another
    // call to flush(), the following steps guard against this:
    // 1. During beforeUpdate, any updated components will be added to the
    //    dirty_components array and will cause a reentrant call to flush(). Because
    //    the flush index is kept outside the function, the reentrant call will pick
    //    up where the earlier call left off and go through all dirty components. The
    //    current_component value is saved and restored so that the reentrant call will
    //    not interfere with the "parent" flush() call.
    // 2. bind:this callbacks cannot trigger new flush() calls.
    // 3. During afterUpdate, any updated components will NOT have their afterUpdate
    //    callback called a second time; the seen_callbacks set, outside the flush()
    //    function, guarantees this behavior.
    const seen_callbacks = new Set();
    let flushidx = 0; // Do *not* move this inside the flush() function
    function flush() {
        const saved_component = current_component;
        do {
            // first, call beforeUpdate functions
            // and update components
            while (flushidx < dirty_components.length) {
                const component = dirty_components[flushidx];
                flushidx++;
                set_current_component(component);
                update(component.$$);
            }
            set_current_component(null);
            dirty_components.length = 0;
            flushidx = 0;
            while (binding_callbacks.length)
                binding_callbacks.pop()();
            // then, once components are updated, call
            // afterUpdate functions. This may cause
            // subsequent updates...
            for (let i = 0; i < render_callbacks.length; i += 1) {
                const callback = render_callbacks[i];
                if (!seen_callbacks.has(callback)) {
                    // ...so guard against infinite loops
                    seen_callbacks.add(callback);
                    callback();
                }
            }
            render_callbacks.length = 0;
        } while (dirty_components.length);
        while (flush_callbacks.length) {
            flush_callbacks.pop()();
        }
        update_scheduled = false;
        seen_callbacks.clear();
        set_current_component(saved_component);
    }
    function update($$) {
        if ($$.fragment !== null) {
            $$.update();
            run_all($$.before_update);
            const dirty = $$.dirty;
            $$.dirty = [-1];
            $$.fragment && $$.fragment.p($$.ctx, dirty);
            $$.after_update.forEach(add_render_callback);
        }
    }
    const outroing = new Set();
    function transition_in(block, local) {
        if (block && block.i) {
            outroing.delete(block);
            block.i(local);
        }
    }
    function mount_component(component, target, anchor, customElement) {
        const { fragment, on_mount, on_destroy, after_update } = component.$$;
        fragment && fragment.m(target, anchor);
        if (!customElement) {
            // onMount happens before the initial afterUpdate
            add_render_callback(() => {
                const new_on_destroy = on_mount.map(run).filter(is_function);
                if (on_destroy) {
                    on_destroy.push(...new_on_destroy);
                }
                else {
                    // Edge case - component was destroyed immediately,
                    // most likely as a result of a binding initialising
                    run_all(new_on_destroy);
                }
                component.$$.on_mount = [];
            });
        }
        after_update.forEach(add_render_callback);
    }
    function destroy_component(component, detaching) {
        const $$ = component.$$;
        if ($$.fragment !== null) {
            run_all($$.on_destroy);
            $$.fragment && $$.fragment.d(detaching);
            // TODO null out other refs, including component.$$ (but need to
            // preserve final state?)
            $$.on_destroy = $$.fragment = null;
            $$.ctx = [];
        }
    }
    function make_dirty(component, i) {
        if (component.$$.dirty[0] === -1) {
            dirty_components.push(component);
            schedule_update();
            component.$$.dirty.fill(0);
        }
        component.$$.dirty[(i / 31) | 0] |= (1 << (i % 31));
    }
    function init$1(component, options, instance, create_fragment, not_equal, props, append_styles, dirty = [-1]) {
        const parent_component = current_component;
        set_current_component(component);
        const $$ = component.$$ = {
            fragment: null,
            ctx: null,
            // state
            props,
            update: noop,
            not_equal,
            bound: blank_object(),
            // lifecycle
            on_mount: [],
            on_destroy: [],
            on_disconnect: [],
            before_update: [],
            after_update: [],
            context: new Map(options.context || (parent_component ? parent_component.$$.context : [])),
            // everything else
            callbacks: blank_object(),
            dirty,
            skip_bound: false,
            root: options.target || parent_component.$$.root
        };
        append_styles && append_styles($$.root);
        let ready = false;
        $$.ctx = instance
            ? instance(component, options.props || {}, (i, ret, ...rest) => {
                const value = rest.length ? rest[0] : ret;
                if ($$.ctx && not_equal($$.ctx[i], $$.ctx[i] = value)) {
                    if (!$$.skip_bound && $$.bound[i])
                        $$.bound[i](value);
                    if (ready)
                        make_dirty(component, i);
                }
                return ret;
            })
            : [];
        $$.update();
        ready = true;
        run_all($$.before_update);
        // `false` as a special case of no DOM component
        $$.fragment = create_fragment ? create_fragment($$.ctx) : false;
        if (options.target) {
            if (options.hydrate) {
                const nodes = children(options.target);
                // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
                $$.fragment && $$.fragment.l(nodes);
                nodes.forEach(detach);
            }
            else {
                // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
                $$.fragment && $$.fragment.c();
            }
            if (options.intro)
                transition_in(component.$$.fragment);
            mount_component(component, options.target, options.anchor, options.customElement);
            flush();
        }
        set_current_component(parent_component);
    }
    /**
     * Base class for Svelte components. Used when dev=false.
     */
    class SvelteComponent {
        $destroy() {
            destroy_component(this, 1);
            this.$destroy = noop;
        }
        $on(type, callback) {
            const callbacks = (this.$$.callbacks[type] || (this.$$.callbacks[type] = []));
            callbacks.push(callback);
            return () => {
                const index = callbacks.indexOf(callback);
                if (index !== -1)
                    callbacks.splice(index, 1);
            };
        }
        $set($$props) {
            if (this.$$set && !is_empty($$props)) {
                this.$$.skip_bound = true;
                this.$$set($$props);
                this.$$.skip_bound = false;
            }
        }
    }

    function dispatch_dev(type, detail) {
        document.dispatchEvent(custom_event(type, Object.assign({ version: '3.48.0' }, detail), { bubbles: true }));
    }
    function append_dev(target, node) {
        dispatch_dev('SvelteDOMInsert', { target, node });
        append(target, node);
    }
    function insert_dev(target, node, anchor) {
        dispatch_dev('SvelteDOMInsert', { target, node, anchor });
        insert(target, node, anchor);
    }
    function detach_dev(node) {
        dispatch_dev('SvelteDOMRemove', { node });
        detach(node);
    }
    function listen_dev(node, event, handler, options, has_prevent_default, has_stop_propagation) {
        const modifiers = options === true ? ['capture'] : options ? Array.from(Object.keys(options)) : [];
        if (has_prevent_default)
            modifiers.push('preventDefault');
        if (has_stop_propagation)
            modifiers.push('stopPropagation');
        dispatch_dev('SvelteDOMAddEventListener', { node, event, handler, modifiers });
        const dispose = listen(node, event, handler, options);
        return () => {
            dispatch_dev('SvelteDOMRemoveEventListener', { node, event, handler, modifiers });
            dispose();
        };
    }
    function attr_dev(node, attribute, value) {
        attr(node, attribute, value);
        if (value == null)
            dispatch_dev('SvelteDOMRemoveAttribute', { node, attribute });
        else
            dispatch_dev('SvelteDOMSetAttribute', { node, attribute, value });
    }
    function set_data_dev(text, data) {
        data = '' + data;
        if (text.wholeText === data)
            return;
        dispatch_dev('SvelteDOMSetData', { node: text, data });
        text.data = data;
    }
    function validate_each_argument(arg) {
        if (typeof arg !== 'string' && !(arg && typeof arg === 'object' && 'length' in arg)) {
            let msg = '{#each} only iterates over array-like objects.';
            if (typeof Symbol === 'function' && arg && Symbol.iterator in arg) {
                msg += ' You can use a spread to convert this iterable into an array.';
            }
            throw new Error(msg);
        }
    }
    function validate_slots(name, slot, keys) {
        for (const slot_key of Object.keys(slot)) {
            if (!~keys.indexOf(slot_key)) {
                console.warn(`<${name}> received an unexpected slot "${slot_key}".`);
            }
        }
    }
    /**
     * Base class for Svelte components with some minor dev-enhancements. Used when dev=true.
     */
    class SvelteComponentDev extends SvelteComponent {
        constructor(options) {
            if (!options || (!options.target && !options.$$inline)) {
                throw new Error("'target' is a required option");
            }
            super();
        }
        $destroy() {
            super.$destroy();
            this.$destroy = () => {
                console.warn('Component was already destroyed'); // eslint-disable-line no-console
            };
        }
        $capture_state() { }
        $inject_state() { }
    }

    /* src/App.svelte generated by Svelte v3.48.0 */
    const file = "src/App.svelte";

    function get_each_context(ctx, list, i) {
    	const child_ctx = ctx.slice();
    	child_ctx[22] = list[i];
    	return child_ctx;
    }

    // (68:2) {:else}
    function create_else_block(ctx) {
    	let label0;
    	let t1;
    	let input0;
    	let t2;
    	let label1;
    	let t4;
    	let input1;
    	let t5;
    	let label2;
    	let t7;
    	let input2;
    	let t8;
    	let label3;
    	let t10;
    	let input3;
    	let t11;
    	let label4;
    	let t13;
    	let input4;
    	let t14;
    	let label5;
    	let t16;
    	let input5;
    	let mounted;
    	let dispose;

    	const block = {
    		c: function create() {
    			label0 = element("label");
    			label0.textContent = "Goal";
    			t1 = space();
    			input0 = element("input");
    			t2 = space();
    			label1 = element("label");
    			label1.textContent = "Num1";
    			t4 = space();
    			input1 = element("input");
    			t5 = space();
    			label2 = element("label");
    			label2.textContent = "Num2";
    			t7 = space();
    			input2 = element("input");
    			t8 = space();
    			label3 = element("label");
    			label3.textContent = "Num3";
    			t10 = space();
    			input3 = element("input");
    			t11 = space();
    			label4 = element("label");
    			label4.textContent = "Num4";
    			t13 = space();
    			input4 = element("input");
    			t14 = space();
    			label5 = element("label");
    			label5.textContent = "Num5";
    			t16 = space();
    			input5 = element("input");
    			attr_dev(label0, "for", "goal");
    			add_location(label0, file, 68, 4, 1455);
    			attr_dev(input0, "type", "number");
    			attr_dev(input0, "id", "goal");
    			add_location(input0, file, 69, 4, 1490);
    			attr_dev(label1, "for", "num1");
    			add_location(label1, file, 70, 4, 1546);
    			attr_dev(input1, "type", "number");
    			attr_dev(input1, "id", "num1");
    			add_location(input1, file, 71, 4, 1581);
    			attr_dev(label2, "for", "num2");
    			add_location(label2, file, 72, 4, 1637);
    			attr_dev(input2, "type", "number");
    			attr_dev(input2, "id", "num2");
    			add_location(input2, file, 73, 4, 1672);
    			attr_dev(label3, "for", "num3");
    			add_location(label3, file, 74, 4, 1728);
    			attr_dev(input3, "type", "number");
    			attr_dev(input3, "id", "num3");
    			add_location(input3, file, 75, 4, 1763);
    			attr_dev(label4, "for", "num4");
    			add_location(label4, file, 76, 4, 1819);
    			attr_dev(input4, "type", "number");
    			attr_dev(input4, "id", "num4");
    			add_location(input4, file, 77, 4, 1854);
    			attr_dev(label5, "for", "num5");
    			add_location(label5, file, 78, 4, 1910);
    			attr_dev(input5, "type", "number");
    			attr_dev(input5, "id", "num5");
    			add_location(input5, file, 79, 4, 1945);
    		},
    		m: function mount(target, anchor) {
    			insert_dev(target, label0, anchor);
    			insert_dev(target, t1, anchor);
    			insert_dev(target, input0, anchor);
    			set_input_value(input0, /*goal*/ ctx[3]);
    			insert_dev(target, t2, anchor);
    			insert_dev(target, label1, anchor);
    			insert_dev(target, t4, anchor);
    			insert_dev(target, input1, anchor);
    			set_input_value(input1, /*num1*/ ctx[4]);
    			insert_dev(target, t5, anchor);
    			insert_dev(target, label2, anchor);
    			insert_dev(target, t7, anchor);
    			insert_dev(target, input2, anchor);
    			set_input_value(input2, /*num2*/ ctx[5]);
    			insert_dev(target, t8, anchor);
    			insert_dev(target, label3, anchor);
    			insert_dev(target, t10, anchor);
    			insert_dev(target, input3, anchor);
    			set_input_value(input3, /*num3*/ ctx[6]);
    			insert_dev(target, t11, anchor);
    			insert_dev(target, label4, anchor);
    			insert_dev(target, t13, anchor);
    			insert_dev(target, input4, anchor);
    			set_input_value(input4, /*num4*/ ctx[7]);
    			insert_dev(target, t14, anchor);
    			insert_dev(target, label5, anchor);
    			insert_dev(target, t16, anchor);
    			insert_dev(target, input5, anchor);
    			set_input_value(input5, /*num5*/ ctx[8]);

    			if (!mounted) {
    				dispose = [
    					listen_dev(input0, "input", /*input0_input_handler*/ ctx[13]),
    					listen_dev(input1, "input", /*input1_input_handler*/ ctx[14]),
    					listen_dev(input2, "input", /*input2_input_handler*/ ctx[15]),
    					listen_dev(input3, "input", /*input3_input_handler*/ ctx[16]),
    					listen_dev(input4, "input", /*input4_input_handler*/ ctx[17]),
    					listen_dev(input5, "input", /*input5_input_handler*/ ctx[18])
    				];

    				mounted = true;
    			}
    		},
    		p: function update(ctx, dirty) {
    			if (dirty & /*goal*/ 8 && to_number(input0.value) !== /*goal*/ ctx[3]) {
    				set_input_value(input0, /*goal*/ ctx[3]);
    			}

    			if (dirty & /*num1*/ 16 && to_number(input1.value) !== /*num1*/ ctx[4]) {
    				set_input_value(input1, /*num1*/ ctx[4]);
    			}

    			if (dirty & /*num2*/ 32 && to_number(input2.value) !== /*num2*/ ctx[5]) {
    				set_input_value(input2, /*num2*/ ctx[5]);
    			}

    			if (dirty & /*num3*/ 64 && to_number(input3.value) !== /*num3*/ ctx[6]) {
    				set_input_value(input3, /*num3*/ ctx[6]);
    			}

    			if (dirty & /*num4*/ 128 && to_number(input4.value) !== /*num4*/ ctx[7]) {
    				set_input_value(input4, /*num4*/ ctx[7]);
    			}

    			if (dirty & /*num5*/ 256 && to_number(input5.value) !== /*num5*/ ctx[8]) {
    				set_input_value(input5, /*num5*/ ctx[8]);
    			}
    		},
    		d: function destroy(detaching) {
    			if (detaching) detach_dev(label0);
    			if (detaching) detach_dev(t1);
    			if (detaching) detach_dev(input0);
    			if (detaching) detach_dev(t2);
    			if (detaching) detach_dev(label1);
    			if (detaching) detach_dev(t4);
    			if (detaching) detach_dev(input1);
    			if (detaching) detach_dev(t5);
    			if (detaching) detach_dev(label2);
    			if (detaching) detach_dev(t7);
    			if (detaching) detach_dev(input2);
    			if (detaching) detach_dev(t8);
    			if (detaching) detach_dev(label3);
    			if (detaching) detach_dev(t10);
    			if (detaching) detach_dev(input3);
    			if (detaching) detach_dev(t11);
    			if (detaching) detach_dev(label4);
    			if (detaching) detach_dev(t13);
    			if (detaching) detach_dev(input4);
    			if (detaching) detach_dev(t14);
    			if (detaching) detach_dev(label5);
    			if (detaching) detach_dev(t16);
    			if (detaching) detach_dev(input5);
    			mounted = false;
    			run_all(dispose);
    		}
    	};

    	dispatch_dev("SvelteRegisterBlock", {
    		block,
    		id: create_else_block.name,
    		type: "else",
    		source: "(68:2) {:else}",
    		ctx
    	});

    	return block;
    }

    // (66:2) {#if useDate}
    function create_if_block(ctx) {
    	let input;
    	let mounted;
    	let dispose;

    	const block = {
    		c: function create() {
    			input = element("input");
    			attr_dev(input, "type", "date");
    			add_location(input, file, 66, 4, 1401);
    		},
    		m: function mount(target, anchor) {
    			insert_dev(target, input, anchor);
    			set_input_value(input, /*date*/ ctx[9]);

    			if (!mounted) {
    				dispose = listen_dev(input, "input", /*input_input_handler*/ ctx[12]);
    				mounted = true;
    			}
    		},
    		p: function update(ctx, dirty) {
    			if (dirty & /*date*/ 512) {
    				set_input_value(input, /*date*/ ctx[9]);
    			}
    		},
    		d: function destroy(detaching) {
    			if (detaching) detach_dev(input);
    			mounted = false;
    			dispose();
    		}
    	};

    	dispatch_dev("SvelteRegisterBlock", {
    		block,
    		id: create_if_block.name,
    		type: "if",
    		source: "(66:2) {#if useDate}",
    		ctx
    	});

    	return block;
    }

    // (83:4) {#each content as item}
    function create_each_block(ctx) {
    	let li;
    	let t_value = /*item*/ ctx[22] + "";
    	let t;

    	const block = {
    		c: function create() {
    			li = element("li");
    			t = text(t_value);
    			add_location(li, file, 83, 6, 2062);
    		},
    		m: function mount(target, anchor) {
    			insert_dev(target, li, anchor);
    			append_dev(li, t);
    		},
    		p: function update(ctx, dirty) {
    			if (dirty & /*content*/ 1 && t_value !== (t_value = /*item*/ ctx[22] + "")) set_data_dev(t, t_value);
    		},
    		d: function destroy(detaching) {
    			if (detaching) detach_dev(li);
    		}
    	};

    	dispatch_dev("SvelteRegisterBlock", {
    		block,
    		id: create_each_block.name,
    		type: "each",
    		source: "(83:4) {#each content as item}",
    		ctx
    	});

    	return block;
    }

    function create_fragment(ctx) {
    	let h1;
    	let t1;
    	let p0;
    	let t2;
    	let a0;
    	let t4;
    	let t5;
    	let p1;
    	let a1;
    	let t7;
    	let p2;
    	let t8;
    	let div;
    	let button;
    	let t9_value = (/*running*/ ctx[1] ? 'Stop' : 'Start') + "";
    	let t9;
    	let t10;
    	let label;
    	let t12;
    	let input;
    	let t13;
    	let t14;
    	let ul;
    	let mounted;
    	let dispose;

    	function select_block_type(ctx, dirty) {
    		if (/*useDate*/ ctx[2]) return create_if_block;
    		return create_else_block;
    	}

    	let current_block_type = select_block_type(ctx);
    	let if_block = current_block_type(ctx);
    	let each_value = /*content*/ ctx[0];
    	validate_each_argument(each_value);
    	let each_blocks = [];

    	for (let i = 0; i < each_value.length; i += 1) {
    		each_blocks[i] = create_each_block(get_each_context(ctx, each_value, i));
    	}

    	const block = {
    		c: function create() {
    			h1 = element("h1");
    			h1.textContent = "Dateo Fun Remover!";
    			t1 = space();
    			p0 = element("p");
    			t2 = text("Check out the ");
    			a0 = element("a");
    			a0.textContent = "for real fun.";
    			t4 = text(" This solver\n  was made in collaboration with Finn McKibbin.");
    			t5 = space();
    			p1 = element("p");
    			a1 = element("a");
    			a1.textContent = "Source code";
    			t7 = space();
    			p2 = element("p");
    			t8 = space();
    			div = element("div");
    			button = element("button");
    			t9 = text(t9_value);
    			t10 = space();
    			label = element("label");
    			label.textContent = "Use Date";
    			t12 = space();
    			input = element("input");
    			t13 = space();
    			if_block.c();
    			t14 = space();
    			ul = element("ul");

    			for (let i = 0; i < each_blocks.length; i += 1) {
    				each_blocks[i].c();
    			}

    			add_location(h1, file, 52, 0, 944);
    			attr_dev(a0, "href", "http://dateo-math-game.com");
    			add_location(a0, file, 54, 16, 992);
    			add_location(p0, file, 53, 0, 972);
    			attr_dev(a1, "href", "https://github.com/Ashwagandhae/dateo-fun-remover");
    			add_location(a1, file, 58, 2, 1118);
    			add_location(p1, file, 57, 0, 1112);
    			add_location(p2, file, 60, 0, 1199);
    			add_location(button, file, 62, 2, 1213);
    			attr_dev(label, "for", "useDate");
    			add_location(label, file, 63, 2, 1279);
    			attr_dev(input, "type", "checkbox");
    			attr_dev(input, "id", "useDate");
    			add_location(input, file, 64, 2, 1319);
    			attr_dev(ul, "class", "content");
    			add_location(ul, file, 81, 2, 2007);
    			add_location(div, file, 61, 0, 1205);
    		},
    		l: function claim(nodes) {
    			throw new Error("options.hydrate only works if the component was compiled with the `hydratable: true` option");
    		},
    		m: function mount(target, anchor) {
    			insert_dev(target, h1, anchor);
    			insert_dev(target, t1, anchor);
    			insert_dev(target, p0, anchor);
    			append_dev(p0, t2);
    			append_dev(p0, a0);
    			append_dev(p0, t4);
    			insert_dev(target, t5, anchor);
    			insert_dev(target, p1, anchor);
    			append_dev(p1, a1);
    			insert_dev(target, t7, anchor);
    			insert_dev(target, p2, anchor);
    			insert_dev(target, t8, anchor);
    			insert_dev(target, div, anchor);
    			append_dev(div, button);
    			append_dev(button, t9);
    			append_dev(div, t10);
    			append_dev(div, label);
    			append_dev(div, t12);
    			append_dev(div, input);
    			input.checked = /*useDate*/ ctx[2];
    			append_dev(div, t13);
    			if_block.m(div, null);
    			append_dev(div, t14);
    			append_dev(div, ul);

    			for (let i = 0; i < each_blocks.length; i += 1) {
    				each_blocks[i].m(ul, null);
    			}

    			if (!mounted) {
    				dispose = [
    					listen_dev(button, "click", /*toggle*/ ctx[10], false, false, false),
    					listen_dev(input, "change", /*input_change_handler*/ ctx[11])
    				];

    				mounted = true;
    			}
    		},
    		p: function update(ctx, [dirty]) {
    			if (dirty & /*running*/ 2 && t9_value !== (t9_value = (/*running*/ ctx[1] ? 'Stop' : 'Start') + "")) set_data_dev(t9, t9_value);

    			if (dirty & /*useDate*/ 4) {
    				input.checked = /*useDate*/ ctx[2];
    			}

    			if (current_block_type === (current_block_type = select_block_type(ctx)) && if_block) {
    				if_block.p(ctx, dirty);
    			} else {
    				if_block.d(1);
    				if_block = current_block_type(ctx);

    				if (if_block) {
    					if_block.c();
    					if_block.m(div, t14);
    				}
    			}

    			if (dirty & /*content*/ 1) {
    				each_value = /*content*/ ctx[0];
    				validate_each_argument(each_value);
    				let i;

    				for (i = 0; i < each_value.length; i += 1) {
    					const child_ctx = get_each_context(ctx, each_value, i);

    					if (each_blocks[i]) {
    						each_blocks[i].p(child_ctx, dirty);
    					} else {
    						each_blocks[i] = create_each_block(child_ctx);
    						each_blocks[i].c();
    						each_blocks[i].m(ul, null);
    					}
    				}

    				for (; i < each_blocks.length; i += 1) {
    					each_blocks[i].d(1);
    				}

    				each_blocks.length = each_value.length;
    			}
    		},
    		i: noop,
    		o: noop,
    		d: function destroy(detaching) {
    			if (detaching) detach_dev(h1);
    			if (detaching) detach_dev(t1);
    			if (detaching) detach_dev(p0);
    			if (detaching) detach_dev(t5);
    			if (detaching) detach_dev(p1);
    			if (detaching) detach_dev(t7);
    			if (detaching) detach_dev(p2);
    			if (detaching) detach_dev(t8);
    			if (detaching) detach_dev(div);
    			if_block.d();
    			destroy_each(each_blocks, detaching);
    			mounted = false;
    			run_all(dispose);
    		}
    	};

    	dispatch_dev("SvelteRegisterBlock", {
    		block,
    		id: create_fragment.name,
    		type: "component",
    		source: "",
    		ctx
    	});

    	return block;
    }

    function instance($$self, $$props, $$invalidate) {
    	let { $$slots: slots = {}, $$scope } = $$props;
    	validate_slots('App', slots, []);
    	let worker;
    	let content = [];
    	let running = false;

    	function start() {
    		$$invalidate(0, content = []);
    		$$invalidate(1, running = true);
    		worker = new Worker('build/worker.js');

    		worker.onmessage = ({ data }) => {
    			if (data.message === 'ready') {
    				worker.postMessage({
    					message: 'start',
    					useDate,
    					goal,
    					num1,
    					num2,
    					num3,
    					num4,
    					num5,
    					date
    				});
    			}

    			if (data.message === 'solution') {
    				$$invalidate(0, content = [...content, data.solution]);
    			}
    		};
    	}

    	function stop() {
    		$$invalidate(1, running = false);
    		worker.terminate();
    	}

    	function toggle() {
    		if (running) {
    			stop();
    		} else {
    			start();
    		}
    	}

    	// Budget rust enum
    	let useDate = true;

    	let goal = 1;
    	let [num1, num2, num3, num4, num5] = [1, 2, 3, 4, 5];
    	let date = new Date().toISOString().slice(0, 10);
    	const writable_props = [];

    	Object.keys($$props).forEach(key => {
    		if (!~writable_props.indexOf(key) && key.slice(0, 2) !== '$$' && key !== 'slot') console.warn(`<App> was created with unknown prop '${key}'`);
    	});

    	function input_change_handler() {
    		useDate = this.checked;
    		$$invalidate(2, useDate);
    	}

    	function input_input_handler() {
    		date = this.value;
    		$$invalidate(9, date);
    	}

    	function input0_input_handler() {
    		goal = to_number(this.value);
    		$$invalidate(3, goal);
    	}

    	function input1_input_handler() {
    		num1 = to_number(this.value);
    		$$invalidate(4, num1);
    	}

    	function input2_input_handler() {
    		num2 = to_number(this.value);
    		$$invalidate(5, num2);
    	}

    	function input3_input_handler() {
    		num3 = to_number(this.value);
    		$$invalidate(6, num3);
    	}

    	function input4_input_handler() {
    		num4 = to_number(this.value);
    		$$invalidate(7, num4);
    	}

    	function input5_input_handler() {
    		num5 = to_number(this.value);
    		$$invalidate(8, num5);
    	}

    	$$self.$capture_state = () => ({
    		onMount,
    		worker,
    		content,
    		running,
    		start,
    		stop,
    		toggle,
    		useDate,
    		goal,
    		num1,
    		num2,
    		num3,
    		num4,
    		num5,
    		date
    	});

    	$$self.$inject_state = $$props => {
    		if ('worker' in $$props) worker = $$props.worker;
    		if ('content' in $$props) $$invalidate(0, content = $$props.content);
    		if ('running' in $$props) $$invalidate(1, running = $$props.running);
    		if ('useDate' in $$props) $$invalidate(2, useDate = $$props.useDate);
    		if ('goal' in $$props) $$invalidate(3, goal = $$props.goal);
    		if ('num1' in $$props) $$invalidate(4, num1 = $$props.num1);
    		if ('num2' in $$props) $$invalidate(5, num2 = $$props.num2);
    		if ('num3' in $$props) $$invalidate(6, num3 = $$props.num3);
    		if ('num4' in $$props) $$invalidate(7, num4 = $$props.num4);
    		if ('num5' in $$props) $$invalidate(8, num5 = $$props.num5);
    		if ('date' in $$props) $$invalidate(9, date = $$props.date);
    	};

    	if ($$props && "$$inject" in $$props) {
    		$$self.$inject_state($$props.$$inject);
    	}

    	return [
    		content,
    		running,
    		useDate,
    		goal,
    		num1,
    		num2,
    		num3,
    		num4,
    		num5,
    		date,
    		toggle,
    		input_change_handler,
    		input_input_handler,
    		input0_input_handler,
    		input1_input_handler,
    		input2_input_handler,
    		input3_input_handler,
    		input4_input_handler,
    		input5_input_handler
    	];
    }

    class App extends SvelteComponentDev {
    	constructor(options) {
    		super(options);
    		init$1(this, options, instance, create_fragment, safe_not_equal, {});

    		dispatch_dev("SvelteRegisterComponent", {
    			component: this,
    			tagName: "App",
    			options,
    			id: create_fragment.name
    		});
    	}
    }

    const init = async () => {
      new App({
        target: document.body,
      });
    };

    init();

})();
//# sourceMappingURL=bundle.js.map
