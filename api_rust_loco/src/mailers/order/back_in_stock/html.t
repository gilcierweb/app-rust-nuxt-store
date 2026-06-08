<html>

<body>
  <h2>Good news, {{name}}!</h2>
  <p>The product you were interested in is back in stock and ready to order.</p>

  <h3>Product Details</h3>
  <table style="width:100%; border-collapse:collapse;">
    <tr style="border-bottom:1px solid #eee;">
      <td><strong>Product:</strong></td>
      <td>{{product_name}}</td>
    </tr>
    {% if variant_name %}
    <tr style="border-bottom:1px solid #eee;">
      <td><strong>Variant:</strong></td>
      <td>{{variant_name}}</td>
    </tr>
    {% endif %}
    {% if price %}
    <tr style="border-bottom:1px solid #eee;">
      <td><strong>Price:</strong></td>
      <td>{{currency}} {{price}}</td>
    </tr>
    {% endif %}
  </table>

  <p style="margin-top:20px;">
    <a href="{{product_url}}" style="display:inline-block; padding:12px 24px; background-color:#4F46E5; color:#ffffff; text-decoration:none; border-radius:6px;">
      Shop Now
    </a>
  </p>

  <p>Stock is limited, so order soon!</p>
  <p>Best regards,<br>The Store Team</p>
</body>

</html>