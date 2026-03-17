const { createElement: h } = React;
const {
  ThemeProvider,
  CssBaseline,
  createTheme,
  Box,
  Stack,
  AppBar,
  Toolbar,
  Typography,
  Container,
  Card,
  CardContent,
  Chip,
  Divider,
  List,
  ListItem,
  ListItemText,
  ListItemButton,
  Button,
  IconButton,
  Checkbox,
  FormControlLabel,
  Drawer,
  Tabs,
  Tab,
  BottomNavigation,
  BottomNavigationAction,
  TextField,
  Select,
  FormControl,
  InputLabel,
  MenuItem,
  Dialog,
  DialogTitle,
  DialogContent,
  DialogActions,
  Fab
} = MaterialUI;

const theme = createTheme({
  palette: {
    mode: 'light',
    primary: { main: '#6750A4' },
    secondary: { main: '#625B71' }
  },
  shape: { borderRadius: 12 }
});

const encoder = new TextEncoder();
const decoder = new TextDecoder('utf-8');

function extractVariant(node) {
  if (!node || typeof node !== 'object') return { type: 'Unknown', value: node };
  const keys = Object.keys(node);
  if (keys.length !== 1) return { type: 'Unknown', value: node };
  return { type: keys[0], value: node[keys[0]] };
}

function colorToCss(color) {
  if (!color) return undefined;
  const { type, value } = extractVariant(color);
  if (type === 'Rgb') {
    const a = value.a / 255;
    return `rgba(${value.r}, ${value.g}, ${value.b}, ${a.toFixed(3)})`;
  }
  if (type === 'Hex') {
    return value.value;
  }
  return undefined;
}

function modifiersToSx(modifiers) {
  if (!modifiers) return {};
  const sx = {};
  if (modifiers.padding !== null && modifiers.padding !== undefined) sx.p = modifiers.padding;
  if (modifiers.padding_horizontal !== null && modifiers.padding_horizontal !== undefined) sx.px = modifiers.padding_horizontal;
  if (modifiers.padding_vertical !== null && modifiers.padding_vertical !== undefined) sx.py = modifiers.padding_vertical;
  if (modifiers.width !== null && modifiers.width !== undefined) sx.width = modifiers.width;
  if (modifiers.height !== null && modifiers.height !== undefined) sx.height = modifiers.height;
  if (modifiers.fill_max_width) sx.width = '100%';
  if (modifiers.fill_max_height) sx.height = '100%';
  if (modifiers.background_color) sx.backgroundColor = colorToCss(modifiers.background_color);
  if (modifiers.border_width) {
    sx.border = `${modifiers.border_width}px solid ${colorToCss(modifiers.border_color) || 'transparent'}`;
  }
  if (modifiers.alpha !== null && modifiers.alpha !== undefined) sx.opacity = modifiers.alpha;
  if (modifiers.corner_radius !== null && modifiers.corner_radius !== undefined) sx.borderRadius = modifiers.corner_radius;
  if (modifiers.offset_x !== null || modifiers.offset_y !== null) {
    sx.transform = `translate(${modifiers.offset_x || 0}px, ${modifiers.offset_y || 0}px)`;
  }
  if (modifiers.z_index !== null && modifiers.z_index !== undefined) sx.zIndex = modifiers.z_index;
  return sx;
}

function iconToLabel(icon) {
  const { type } = extractVariant(icon);
  return type === 'Unknown' ? 'Icon' : type;
}

function memoryBytesToDataUrl(bytes) {
  if (!bytes || !bytes.length) return null;
  let binary = '';
  const chunk = 0x8000;
  for (let i = 0; i < bytes.length; i += chunk) {
    binary += String.fromCharCode(...bytes.slice(i, i + chunk));
  }
  return `data:application/octet-stream;base64,${btoa(binary)}`;
}

function millisToDateInput(millis) {
  if (!millis) return '';
  const date = new Date(millis);
  const yyyy = date.getFullYear();
  const mm = String(date.getMonth() + 1).padStart(2, '0');
  const dd = String(date.getDate()).padStart(2, '0');
  return `${yyyy}-${mm}-${dd}`;
}

function dateInputToMillis(value) {
  if (!value) return '';
  const date = new Date(value);
  return date.getTime();
}

function timeToInput(hour, minute) {
  if (hour === null || hour === undefined) return '';
  const hh = String(hour).padStart(2, '0');
  const mm = String(minute ?? 0).padStart(2, '0');
  return `${hh}:${mm}`;
}

function renderNode(node, ctx) {
  if (!node) return null;
  const { type, value } = extractVariant(node);
  const children = (list) => (list || []).map((child, index) => h(React.Fragment, { key: index }, renderNode(child, ctx)));
  const sx = modifiersToSx(value?.modifiers);

  switch (type) {
    case 'Scaffold': {
      const appBar = (value.app_bar || [])[0];
      const drawer = (value.drawer || [])[0];
      const rail = (value.rail || [])[0];
      const body = (value.body || [])[0];
      const bottomBar = (value.bottom_bar || [])[0];
      const fab = (value.floating_action_button || [])[0];
      return h(Box, { sx: { minHeight: '100vh', display: 'flex', flexDirection: 'column', ...sx } },
        appBar && renderNode(appBar, ctx),
        h(Box, { sx: { flex: 1, display: 'flex' } },
          drawer && renderNode(drawer, ctx),
          rail && renderNode(rail, ctx),
          h(Box, { sx: { flex: 1, p: 2 } }, body && renderNode(body, ctx))
        ),
        bottomBar && renderNode(bottomBar, ctx),
        fab && h(Box, { sx: { position: 'fixed', right: 24, bottom: 24 } }, renderNode(fab, ctx))
      );
    }
    case 'AppBar': {
      const titleColor = colorToCss(value.options?.title_color);
      const barColor = colorToCss(value.options?.container_color);
      return h(AppBar, { position: 'static', sx: { backgroundColor: barColor } },
        h(Toolbar, null,
          value.leading?.length ? h(Box, { sx: { mr: 2, display: 'flex', gap: 1 } }, children(value.leading)) : null,
          h(Typography, { variant: 'h6', sx: { color: titleColor || 'inherit' } }, value.title)
        )
      );
    }
    case 'NavigationBar': {
      const selectedIndex = value.destinations.findIndex((d) => d.selected);
      return h(BottomNavigation, {
        showLabels: value.options?.always_show_label ?? true,
        value: selectedIndex >= 0 ? selectedIndex : 0,
        onChange: (_, idx) => {
          const dest = value.destinations[idx];
          if (dest) ctx.dispatchAction(dest.action_id);
        }
      },
      value.destinations.map((dest, idx) => h(BottomNavigationAction, {
        key: `${dest.label}-${idx}`,
        label: dest.label,
        icon: h('span', null, iconToLabel(dest.icon))
      }))
      );
    }
    case 'NavigationDrawer': {
      const variantMap = {
        Modal: 'temporary',
        Dismissible: 'persistent',
        Permanent: 'permanent'
      };
      const drawerVariant = variantMap[extractVariant(value.drawer_type).type] || 'temporary';
      return h(Drawer, { variant: drawerVariant, open: true, sx: { '& .MuiDrawer-paper': { width: 280 } } },
        h(Box, { sx: { p: 2 } },
          value.title ? h(Typography, { variant: 'h6', sx: { mb: 1 } }, value.title) : null,
          h(List, null,
            value.destinations.map((dest, idx) => h(ListItemButton, {
              key: `${dest.label}-${idx}`,
              selected: dest.selected,
              onClick: () => ctx.dispatchAction(dest.action_id)
            },
            h(ListItemText, { primary: dest.label, secondary: dest.badge || undefined })
            ))
          )
        )
      );
    }
    case 'NavigationRail': {
      return h(Box, { sx: { width: 96, borderRight: '1px solid #ddd' } },
        h(List, null,
          value.destinations.map((dest, idx) => h(ListItemButton, {
            key: `${dest.label}-${idx}`,
            selected: dest.selected,
            onClick: () => ctx.dispatchAction(dest.action_id)
          },
          h(ListItemText, { primary: value.options?.always_show_label ? dest.label : '', secondary: iconToLabel(dest.icon) })
          ))
        )
      );
    }
    case 'Tabs': {
      const selectedIndex = value.destinations.findIndex((d) => d.selected);
      return h(Tabs, {
        value: selectedIndex >= 0 ? selectedIndex : 0,
        variant: value.options?.scrollable ? 'scrollable' : 'standard',
        onChange: (_, idx) => {
          const dest = value.destinations[idx];
          if (dest) ctx.dispatchAction(dest.action_id);
        }
      },
      value.destinations.map((dest, idx) => h(Tab, {
        key: `${dest.label}-${idx}`,
        label: dest.label,
        icon: dest.icon ? h('span', null, iconToLabel(dest.icon)) : undefined
      }))
      );
    }
    case 'Column':
      return h(Stack, { sx, spacing: 1 }, children(value.children));
    case 'Row':
      return h(Stack, { sx, direction: 'row', spacing: 1, alignItems: 'center' }, children(value.children));
    case 'Stack':
      return h(Box, { sx }, children(value.children));
    case 'Scroll':
      return h(Box, { sx: { overflow: 'auto', ...sx } }, children(value.child));
    case 'ListView':
      return h(List, { sx }, children(value.items));
    case 'ListItem': {
      const disabled = value.options?.enabled === false || value.modifiers?.enabled === false;
      const content = h(ListItemText, {
        primary: value.headline,
        secondary: value.supporting_text || value.overline_text || undefined
      });
      if (value.action_id) {
        return h(ListItem, { disablePadding: true, sx },
          h(ListItemButton, { disabled, onClick: () => ctx.dispatchAction(value.action_id) }, content)
        );
      }
      return h(ListItem, { sx }, content);
    }
    case 'Divider': {
      const vertical = value.options?.vertical ?? false;
      return h(Divider, {
        orientation: vertical ? 'vertical' : 'horizontal',
        sx: {
          bgcolor: colorToCss(value.options?.color),
          thickness: value.options?.thickness,
          ml: value.options?.inset_start,
          mr: value.options?.inset_end,
          ...sx
        }
      });
    }
    case 'Text':
      return h(Typography, { sx: { fontSize: value.sp_size, ...sx } }, value.text);
    case 'TextField': {
      const styleType = extractVariant(value.style).type;
      const variant = styleType === 'Filled' ? 'filled' : 'outlined';
      return h(TextField, {
        label: value.label,
        value: value.value,
        variant,
        placeholder: value.options?.placeholder || undefined,
        helperText: value.error_text || value.options?.supporting_text || undefined,
        disabled: value.options?.enabled === false,
        InputProps: { readOnly: value.options?.read_only === true },
        type: value.options?.is_password ? 'password' : 'text',
        multiline: !value.options?.single_line,
        maxRows: value.options?.single_line ? 1 : value.options?.max_lines,
        onChange: (event) => ctx.dispatchActionWithString(value.on_change_action_id, event.target.value),
        sx
      });
    }
    case 'Menu': {
      return h(Card, { sx },
        h(CardContent, null,
          h(Typography, { variant: 'subtitle1', sx: { mb: 1 } }, value.label),
          h(Stack, { spacing: 1 },
            value.items.map((item, idx) => h(Button, {
              key: `${item.label}-${idx}`,
              variant: 'text',
              disabled: item.enabled === false,
              onClick: () => {
                const actionId = value.action_ids[idx];
                if (actionId) ctx.dispatchAction(actionId);
              }
            }, item.label))
          )
        )
      );
    }
    case 'DropdownField': {
      return h(FormControl, { fullWidth: true, sx },
        h(InputLabel, null, value.label),
        h(Select, {
          value: value.value,
          label: value.label,
          onChange: (event) => ctx.dispatchActionWithString(value.on_change_action_id, event.target.value),
          disabled: value.options?.enabled === false
        },
        value.options_list.map((opt, idx) => h(MenuItem, { key: `${opt}-${idx}`, value: opt }, opt))
        )
      );
    }
    case 'Button': {
      const styleType = extractVariant(value.style).type;
      const variant = styleType === 'Outlined' ? 'outlined' : styleType === 'Text' ? 'text' : 'contained';
      return h(Button, {
        variant,
        disabled: value.options?.enabled === false,
        onClick: () => ctx.dispatchAction(value.action_id),
        sx
      }, value.content?.length ? children(value.content) : 'Button');
    }
    case 'IconButton':
      return h(IconButton, {
        onClick: () => ctx.dispatchAction(value.action_id),
        disabled: value.options?.enabled === false,
        sx
      }, iconToLabel(value.icon));
    case 'Card': {
      const clickable = !!value.action_id;
      return h(Card, {
        sx: { cursor: clickable ? 'pointer' : 'default', ...sx },
        onClick: clickable ? () => ctx.dispatchAction(value.action_id) : undefined
      },
      h(CardContent, null, children(value.children))
      );
    }
    case 'Checkbox':
      return h(FormControlLabel, {
        sx,
        control: h(Checkbox, {
          checked: value.checked,
          disabled: value.enabled === false,
          onChange: () => ctx.dispatchAction(value.action_id)
        }),
        label: ''
      });
    case 'Chip':
      return h(Chip, {
        label: value.label,
        onClick: () => ctx.dispatchAction(value.action_id),
        onDelete: value.close_action_id ? () => ctx.dispatchAction(value.close_action_id) : undefined,
        sx
      });
    case 'Fab': {
      return h(Fab, {
        variant: value.label ? 'extended' : 'circular',
        onClick: () => ctx.dispatchAction(value.action_id),
        sx
      }, value.label || iconToLabel(value.icon));
    }
    case 'Image': {
      const sourceVariant = extractVariant(value.source);
      let src = '';
      if (sourceVariant.type === 'Asset') src = `./assets/${sourceVariant.value.name}`;
      if (sourceVariant.type === 'Network') src = sourceVariant.value.url;
      if (sourceVariant.type === 'File') src = sourceVariant.value.path;
      if (sourceVariant.type === 'Memory') src = memoryBytesToDataUrl(sourceVariant.value.data);
      return h('img', { src, style: { maxWidth: '100%', ...sx } });
    }
    case 'Dialog': {
      return h(Dialog, { open: true },
        value.title ? h(DialogTitle, null, value.title) : null,
        h(DialogContent, null, h(Typography, null, value.text)),
        h(DialogActions, null,
          value.dismiss_label && value.dismiss_action_id ? h(Button, {
            onClick: () => ctx.dispatchAction(value.dismiss_action_id)
          }, value.dismiss_label) : null,
          h(Button, { onClick: () => ctx.dispatchAction(value.confirm_action_id) }, value.confirm_label)
        )
      );
    }
    case 'FullscreenDialog': {
      return h(Dialog, { open: true, fullScreen: true },
        h(DialogTitle, null, value.title),
        h(DialogContent, null, children(value.content)),
        h(DialogActions, null,
          h(Button, { onClick: () => ctx.dispatchAction(value.dismiss_action_id) }, value.dismiss_label),
          value.confirm_label && value.confirm_action_id ? h(Button, {
            onClick: () => ctx.dispatchAction(value.confirm_action_id)
          }, value.confirm_label) : null
        )
      );
    }
    case 'DatePickerDialog':
      return h(DatePickerDialog, { value, ctx });
    case 'DateRangePickerDialog':
      return h(DateRangePickerDialog, { value, ctx });
    case 'TimePickerDialog':
      return h(TimePickerDialog, { value, ctx });
    default:
      return h(Card, { sx: { p: 2, ...sx } },
        h(Typography, { color: 'text.secondary' }, `Unsupported node: ${type}`)
      );
  }
}

function DatePickerDialog({ value, ctx }) {
  const [dateValue, setDateValue] = React.useState(millisToDateInput(value.initial_selected_millis));
  return h(Dialog, { open: true },
    value.title ? h(DialogTitle, null, value.title) : null,
    h(DialogContent, null,
      h(TextField, {
        type: 'date',
        value: dateValue,
        onChange: (event) => setDateValue(event.target.value)
      })
    ),
    h(DialogActions, null,
      value.dismiss_label && value.dismiss_action_id ? h(Button, {
        onClick: () => ctx.dispatchAction(value.dismiss_action_id)
      }, value.dismiss_label) : null,
      h(Button, {
        onClick: () => ctx.dispatchActionWithString(value.confirm_action_id, `${dateInputToMillis(dateValue)}`)
      }, value.confirm_label)
    )
  );
}

function DateRangePickerDialog({ value, ctx }) {
  const [start, setStart] = React.useState(millisToDateInput(value.initial_start_millis));
  const [end, setEnd] = React.useState(millisToDateInput(value.initial_end_millis));
  return h(Dialog, { open: true },
    value.title ? h(DialogTitle, null, value.title) : null,
    h(DialogContent, null,
      h(Stack, { spacing: 2 },
        h(TextField, { type: 'date', value: start, onChange: (e) => setStart(e.target.value) }),
        h(TextField, { type: 'date', value: end, onChange: (e) => setEnd(e.target.value) })
      )
    ),
    h(DialogActions, null,
      value.dismiss_label && value.dismiss_action_id ? h(Button, {
        onClick: () => ctx.dispatchAction(value.dismiss_action_id)
      }, value.dismiss_label) : null,
      h(Button, {
        onClick: () => ctx.dispatchActionWithString(
          value.confirm_action_id,
          `${dateInputToMillis(start)}|${dateInputToMillis(end)}`
        )
      }, value.confirm_label)
    )
  );
}

function TimePickerDialog({ value, ctx }) {
  const [timeValue, setTimeValue] = React.useState(timeToInput(value.initial_hour, value.initial_minute));
  return h(Dialog, { open: true },
    value.title ? h(DialogTitle, null, value.title) : null,
    h(DialogContent, null,
      h(TextField, {
        type: 'time',
        value: timeValue,
        onChange: (event) => setTimeValue(event.target.value)
      })
    ),
    h(DialogActions, null,
      value.dismiss_label && value.dismiss_action_id ? h(Button, {
        onClick: () => ctx.dispatchAction(value.dismiss_action_id)
      }, value.dismiss_label) : null,
      h(Button, {
        onClick: () => ctx.dispatchActionWithString(value.confirm_action_id, timeValue)
      }, value.confirm_label)
    )
  );
}

function App() {
  const [status, setStatus] = React.useState('Loading Rust WASM...');
  const [tree, setTree] = React.useState(null);
  const wasmRef = React.useRef(null);

  const getStringFromWasm = React.useCallback((ptr, len) => {
    if (!ptr || !len) return '';
    return decoder.decode(new Uint8Array(wasmRef.current.exports.memory.buffer, ptr, len));
  }, []);

  const refreshTree = React.useCallback(() => {
    const wasm = wasmRef.current;
    if (!wasm) return;
    const ptr = wasm.exports.padauk_web_render_root_ptr();
    const len = wasm.exports.padauk_web_render_root_len();
    const view = new Uint8Array(wasm.exports.memory.buffer, ptr, len);
    const data = new Uint8Array(view);
    wasm.exports.padauk_web_render_root_free();
    const json = decoder.decode(data);
    try {
      setTree(JSON.parse(json));
    } catch (err) {
      setStatus(`Failed to parse UI tree: ${err.message}`);
    }
  }, []);

  const dispatchAction = React.useCallback((id) => {
    const wasm = wasmRef.current;
    if (!wasm || !id) return;
    const bytes = encoder.encode(id);
    const ptr = wasm.exports.padauk_web_alloc(bytes.length);
    new Uint8Array(wasm.exports.memory.buffer, ptr, bytes.length).set(bytes);
    wasm.exports.padauk_web_dispatch_action(ptr, bytes.length);
    wasm.exports.padauk_web_dealloc(ptr, bytes.length);
    refreshTree();
  }, [refreshTree]);

  const dispatchActionWithString = React.useCallback((id, payload) => {
    const wasm = wasmRef.current;
    if (!wasm || !id) return;
    const idBytes = encoder.encode(id);
    const payloadBytes = encoder.encode(payload ?? '');
    const idPtr = wasm.exports.padauk_web_alloc(idBytes.length);
    const payloadPtr = wasm.exports.padauk_web_alloc(payloadBytes.length);
    new Uint8Array(wasm.exports.memory.buffer, idPtr, idBytes.length).set(idBytes);
    new Uint8Array(wasm.exports.memory.buffer, payloadPtr, payloadBytes.length).set(payloadBytes);
    wasm.exports.padauk_web_dispatch_action_with_string(idPtr, idBytes.length, payloadPtr, payloadBytes.length);
    wasm.exports.padauk_web_dealloc(idPtr, idBytes.length);
    wasm.exports.padauk_web_dealloc(payloadPtr, payloadBytes.length);
    refreshTree();
  }, [refreshTree]);

  React.useEffect(() => {
    fetch('./app.wasm')
      .then(async (response) => {
        if (!response.ok) throw new Error(`HTTP ${response.status}`);
        const bytes = await response.arrayBuffer();
        const imports = {
          __wbindgen_placeholder__: {
            __wbindgen_describe() {},
            __wbindgen_object_drop_ref() {},
            __wbg_getRandomValues_9c5c1b115e142bb8: (ptr, len) => {
              const view = new Uint8Array(wasmRef.current.exports.memory.buffer, ptr, len);
              crypto.getRandomValues(view);
            },
            __wbg___wbindgen_throw_be289d5034ed271b: (ptr, len) => {
              throw new Error(getStringFromWasm(ptr, len));
            }
          },
          __wbindgen_externref_xform__: {
            __wbindgen_externref_table_grow: (delta) => {
              return wasmRef.current.exports.__wbindgen_externref_table.grow(delta);
            },
            __wbindgen_externref_table_set_null: (idx) => {
              wasmRef.current.exports.__wbindgen_externref_table.set(idx, undefined);
            }
          }
        };
        const result = await WebAssembly.instantiate(bytes, imports);
        wasmRef.current = result.instance;
        wasmRef.current.exports.padauk_web_init();
        setStatus('Rust WASM loaded. Rendering UI...');
        refreshTree();
      })
      .catch((error) => {
        setStatus(`WASM not available yet (${error.message}). Run: padauk build web`);
      });
  }, [refreshTree]);

  const ctx = { dispatchAction, dispatchActionWithString };

  return h(ThemeProvider, { theme },
    h(CssBaseline),
    h(Container, { maxWidth: false, disableGutters: true },
      tree ? renderNode(tree, ctx) : h(Box, { sx: { p: 3 } }, h(Chip, { color: 'primary', label: status }))
    )
  );
}

ReactDOM.createRoot(document.getElementById('root')).render(h(App));
