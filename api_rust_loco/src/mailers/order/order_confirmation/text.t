Thank you for your order, {{name}}!

Order Number: {{order_number}}
Date: {{order_date}}

Items:
{% for item in items %}- {{item.name}} x {{item.quantity}} = {{currency}} {{item.total}}
{% endfor %}

Subtotal: {{currency}} {{subtotal}}
Shipping: {{currency}} {{shipping_amount}}
{% if discount_amount != "0.00" %}Discount: -{{currency}} {{discount_amount}}
{% endif %}Total: {{currency}} {{total_amount}}

You will receive another email when your order has shipped.

Best regards,
The Store Team