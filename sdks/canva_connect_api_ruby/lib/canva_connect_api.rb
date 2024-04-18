# Generated by Sideko (sideko.dev)
# frozen_string_literal: true
require 'json'
require 'http'

class RequestError < StandardError
  attr_reader :status_code, :method, :url, :data, :response

  def initialize(method, url, response)
    @status_code = response.status
    @method = method
    @url = url
    begin
      @data = response.parse
    rescue
      @data = nil
    end
    @response = response

    super("received #{status_code} from #{method} #{url} with #{message}")
  end
end

module AuthProvider
  def add_auth(http_client, req_kwargs)
    raise NotImplementedError, "You must implement the 'add_auth' method"
  end
end

class AuthBasic
  include AuthProvider

  attr_accessor :username, :password

  def initialize(username: nil, password: nil)
    @username = username
    @password = password
  end

  def add_auth(http_client, req_kwargs)
    if !@username.nil? && !@password.nil?
      http_client = http_client.basic_auth(user: @username, pass: @password)
    end

    return http_client, req_kwargs
  end
end

class AuthBearer
  include AuthProvider

  attr_accessor :val

  def initialize(val: nil)
    @val = val
  end

  def add_auth(http_client, req_kwargs)
    if !@val.nil?
      headers = req_kwargs.fetch(:headers, {})
      headers["Authorization"] = "Bearer " + @val
      req_kwargs[:headers] = headers
    end

    return http_client, req_kwargs
  end
end

class AuthKeyQuery
  include AuthProvider

  attr_accessor :query_name, :val

  def initialize(query_name: nil, val: nil)
    @query_name = query_name
    @val = val
  end

  def add_auth(http_client, req_kwargs)
    if !val.nil?
      params = req_kwargs.fetch(:params, {})
      params[query_name] = val
      req_kwargs[:params] = params
    end

    return http_client, req_kwargs
  end
end

class AuthKeyHeader
  include AuthProvider

  attr_accessor :header_name, :val

  def initialize(header_name: nil, val: nil)
    @header_name = header_name
    @val = val
  end

  def add_auth(http_client, req_kwargs)
    if !@val.nil?
      headers = req_kwargs.fetch(:headers, {})
      headers[@header_name] = @val
      req_kwargs[:headers] = headers
    end

    return http_client, req_kwargs
  end
end

class AuthKeyCookie
  include AuthProvider

  attr_accessor :cookie_name, :val

  def initialize(cookie_name: nil, val: nil)
    @cookie_name = cookie_name
    @val = val
  end

  def add_auth(http_client, req_kwargs)
    if !@val.nil?
      http_client = http_client.cookies(@cookie_name => @val)
    end

    return http_client, req_kwargs
  end
end

class Client
  def initialize(bearer_auth: nil, base_url: 'https://api.canva.com/rest/v1')
    @_base_url = base_url
    # register auth providers
    @_auth = {}
    @_auth[:"bearerAuth"] = AuthBearer.new(val: bearer_auth)
  end

  def _client_with_auth(auth_names, **req_kwargs)
    http_client = HTTP
    auth_names.each do |auth_name|
      provider = @_auth[auth_name]
      http_client, req_kwargs = provider.add_auth(http_client, req_kwargs) if provider
    end
    
    return http_client, req_kwargs
  end


  def list_all_designs(continuation: nil, ownership: nil, query: nil, sort_by: nil)
    _url = @_base_url + "/designs"
    _kwargs = {
      params: {}
    }
    if continuation != nil
      _kwargs[:params][:"continuation"] = continuation
    end
    if ownership != nil
      _kwargs[:params][:"ownership"] = ownership
    end
    if query != nil
      _kwargs[:params][:"query"] = query
    end
    if sort_by != nil
      _kwargs[:params][:"sort_by"] = sort_by
    end
    _http_client, _req_kwargs = self._client_with_auth(
      [:"bearerAuth", ],
      **_kwargs
    )

    
    _response = _http_client.get(
      _url,
      **_req_kwargs
    )

    # Raise if not expected success code
    if _response.status != 200
      raise RequestError.new(
        method="get",
        url=_url,
        response=_response
      )
    end

    return _response.body.empty? ? '' : _response.parse
  end

end
